mod web_driver;

use std::any::Any;
use std::borrow::Cow;
use std::ffi::OsString;
use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Deref;
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, anyhow};
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, delete, get, post};
use axum::{Json, Router, extract};
use clap::{Args, Parser, Subcommand};
use hashbrown::HashMap;
use js_bindgen_shared::{AtomicFlag, WebDriver, WebDriverKind, WebDriverLocation};
use mime::APPLICATION_JSON;
use reqwest::Request;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::Mutex;
use tower_http::catch_panic::CatchPanicLayer;
use url::Url;
use xxhash_rust::xxh3::Xxh3Default;

use crate::web_driver::{Client, IntoAxum, StatusResponse, WebDriverErrorKind, WebDriverResponse};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[arg(short, long)]
	port: Option<u16>,
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	Chrome(LocalArgs),
	Edge(LocalArgs),
	Gecko(LocalArgs),
	Safari(LocalArgs),
	Remote { url: Url },
}

#[derive(Args)]
struct LocalArgs {
	path: Option<PathBuf>,
	#[arg(last = true)]
	args: Option<Vec<OsString>>,
}

struct AppState {
	client: Client,
	pool: DriverPool,
}

#[expect(clippy::large_enum_variant, reason = "not in a collection")]
enum DriverPool {
	Shared {
		driver: WebDriver,
		available: Mutex<HashMap<u64, Vec<(String, Bytes)>>>,
		active: Mutex<HashMap<String, (u64, Bytes)>>,
	},
	Single {
		path: Cow<'static, Path>,
		args: Vec<OsString>,
		available: Mutex<HashMap<u64, Vec<(String, Driver)>>>,
		active: Mutex<HashMap<String, (u64, Driver)>>,
	},
}

struct Driver {
	driver: WebDriver,
	response: Bytes,
}

#[tokio::main]
async fn main() -> Result<()> {
	let cli = Cli::parse();

	let listener = TcpListener::bind(SocketAddrV4::new(
		Ipv4Addr::LOCALHOST,
		cli.port.unwrap_or_default(),
	))
	.await?;

	println!("Server listening on: {}", listener.local_addr()?);

	let shutdown = Arc::new(AtomicFlag::new());
	let state = Arc::new(AppState {
		client: Client::new(),
		pool: DriverPool::from_command(cli.command).await?,
	});

	let app = Router::new()
		.route("/status", get(handle_status))
		.route("/session", post(create_session))
		.route("/session/{id}", delete(delete_session))
		.route("/session/{id}/{*path}", any(proxy_request))
		.layer(CatchPanicLayer::custom({
			let shutdown = Arc::clone(&shutdown);
			move |error: Box<dyn Any + Send>| -> Response {
				shutdown.signal();
				panic::resume_unwind(error)
			}
		}))
		.with_state(Arc::clone(&state));

	let server = tokio::spawn(
		axum::serve(listener, app)
			.with_graceful_shutdown({
				let shutdown = Arc::clone(&shutdown);
				async move {
					shutdown.deref().await;
				}
			})
			.into_future(),
	);

	println!("Shutdown via CTRL-C\n");

	signal::ctrl_c().await?;
	shutdown.signal();
	server.await??;

	Arc::into_inner(state).unwrap().shutdown().await?;

	Ok(())
}

async fn handle_status(State(state): State<Arc<AppState>>) -> Result<Response, Response> {
	match &state.pool {
		DriverPool::Shared { driver, .. } => state.client.status(driver).await,
		DriverPool::Single { .. } => Ok(Json(WebDriverResponse {
			value: StatusResponse {
				ready: true,
				message: String::new(),
			},
		})
		.into_response()),
	}
}

async fn create_session(
	State(state): State<Arc<AppState>>,
	body: Bytes,
) -> Result<Response, Response> {
	let mut hasher = Xxh3Default::new();
	body.hash(&mut hasher);
	let caps_hash = hasher.digest();

	let response = match &state.pool {
		DriverPool::Shared {
			driver,
			available,
			active,
		} => {
			let available = available
				.lock()
				.await
				.get_mut(&caps_hash)
				.and_then(Vec::pop);

			if let Some((session_id, response)) = available {
				active
					.lock()
					.await
					.insert(session_id, (caps_hash, response.clone()));
				response
			} else {
				let (session_id, response) = state.client.create_session(driver, body).await?;
				println!("Created new WebDriver session at {}", driver.url());

				active
					.lock()
					.await
					.insert(session_id, (caps_hash, response.clone()));

				response
			}
		}
		DriverPool::Single {
			path,
			args,
			available,
			active,
		} => {
			let available = available
				.lock()
				.await
				.get_mut(&caps_hash)
				.and_then(Vec::pop);

			if let Some((session_id, driver)) = available {
				let response = driver.response.clone();
				active.lock().await.insert(session_id, (caps_hash, driver));

				response
			} else {
				let driver = WebDriver::run_local(path, args).await.unwrap();
				let (session_id, response) = state.client.create_session(&driver, body).await?;

				active.lock().await.insert(
					session_id,
					(
						caps_hash,
						Driver {
							driver,
							response: response.clone(),
						},
					),
				);

				response
			}
		}
	};

	let mut response = Response::new(response.into());
	response.headers_mut().insert(
		CONTENT_TYPE,
		HeaderValue::from_static(APPLICATION_JSON.as_ref()),
	);

	Ok(response)
}

async fn delete_session(
	State(state): State<Arc<AppState>>,
	extract::Path(id): extract::Path<String>,
) -> Result<(), Response> {
	match &state.pool {
		DriverPool::Shared {
			driver,
			available,
			active,
		} => {
			let Some((caps_hash, response)) = active.lock().await.remove(&id) else {
				return Err(invalid_session_id(&id));
			};
			state.client.navigate_to(driver, &id, "about:blank").await?;

			available
				.lock()
				.await
				.entry(caps_hash)
				.or_default()
				.push((id, response));
		}
		DriverPool::Single {
			available, active, ..
		} => {
			let Some((caps_hash, driver)) = active.lock().await.remove(&id) else {
				return Err(invalid_session_id(&id));
			};
			state
				.client
				.navigate_to(&driver.driver, &id, "about:blank")
				.await?;

			available
				.lock()
				.await
				.entry(caps_hash)
				.or_default()
				.push((id, driver));
		}
	}

	Ok(())
}

async fn proxy_request(
	State(state): State<Arc<AppState>>,
	extract::Path((id, path)): extract::Path<(String, String)>,
	method: Method,
	body: Bytes,
) -> Result<Response, Response> {
	let mut url = match &state.pool {
		DriverPool::Shared { driver, .. } => driver.url().clone(),
		DriverPool::Single { active, .. } => {
			if let Some((_, driver)) = active.lock().await.get(&id) {
				driver.driver.url().clone()
			} else {
				return Err(invalid_session_id(&id));
			}
		}
	};

	let mut segments = url.path_segments_mut().unwrap();
	segments.push("session");
	segments.push(&id);
	segments.push(&path);
	drop(segments);

	let mut request = Request::new(method, url);
	*request.body_mut() = Some(body.into());
	let response = state
		.client
		.execute(request)
		.await
		.map_err(IntoAxum::into_response)?;

	Ok(Response::from(response).map(Body::new))
}

impl AppState {
	async fn shutdown(self) -> Result<()> {
		let mut errors = false;

		match self.pool {
			DriverPool::Shared {
				driver,
				available,
				active,
			} => {
				for id in available
					.into_inner()
					.into_values()
					.flatten()
					.map(|(id, _)| id)
					.chain(active.into_inner().into_keys())
				{
					if let Err(error) = self.client.delete_session(&driver, &id).await {
						eprintln!("{error}\n");
						errors = true;
					}
				}

				if let Err(error) = driver.shutdown().await {
					eprintln!("{error}\n");
					errors = true;
				}
			}
			DriverPool::Single {
				available, active, ..
			} => {
				for (id, driver) in available.into_inner().into_values().flatten().chain(
					active
						.into_inner()
						.into_iter()
						.map(|(id, (_, driver))| (id, driver)),
				) {
					if let Err(error) = self.client.delete_session(&driver.driver, &id).await {
						eprintln!("{error}\n");
						errors = true;
					}

					if let Err(error) = driver.driver.shutdown().await {
						eprintln!("{error}\n");
						errors = true;
					}
				}
			}
		}

		if errors {
			Err(anyhow!("encountered error(s)"))
		} else {
			Ok(())
		}
	}
}

fn invalid_session_id(id: &str) -> Response {
	WebDriverErrorKind::InvalidSessionId.to_response(format!("No active session with ID {id}"))
}

impl DriverPool {
	async fn from_command(command: Command) -> Result<Self> {
		match command {
			Command::Chrome(args) => Self::local(WebDriverKind::Chrome, args).await,
			Command::Edge(args) => Self::local(WebDriverKind::Edge, args).await,
			Command::Gecko(args) => Self::local(WebDriverKind::Gecko, args).await,
			Command::Safari(args) => Self::local(WebDriverKind::Safari, args).await,
			Command::Remote { url } => Ok(Self::Shared {
				driver: WebDriver::run(WebDriverLocation::Remote(url)).await?,
				available: Mutex::new(HashMap::new()),
				active: Mutex::new(HashMap::new()),
			}),
		}
	}

	async fn local(kind: WebDriverKind, args: LocalArgs) -> Result<Self> {
		let path = args
			.path
			.map_or(Cow::Borrowed(Path::new(kind.to_binary())), Cow::Owned);
		let args = args.args.unwrap_or_default();

		if kind.multi_session_support() {
			Ok(Self::Shared {
				driver: WebDriver::run(WebDriverLocation::Local { path, args }).await?,
				available: Mutex::new(HashMap::new()),
				active: Mutex::new(HashMap::new()),
			})
		} else {
			Ok(Self::Single {
				path,
				args,
				available: Mutex::new(HashMap::new()),
				active: Mutex::new(HashMap::new()),
			})
		}
	}
}
