use std::any::Any;
use std::borrow::Cow;
use std::ffi::OsString;
use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Deref;
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, delete, get, post};
use axum::{Json, Router, extract};
use clap::{Args, Parser, Subcommand};
use hashbrown::HashMap;
use js_bindgen_shared::{AtomicFlag, WebDriver, WebDriverKind, WebDriverLocation};
use mime::APPLICATION_JSON;
use reqwest::{Client, Error, Request};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::Mutex;
use tower_http::catch_panic::CatchPanicLayer;
use url::Url;
use xxhash_rust::xxh3::Xxh3Default;

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

#[derive(Clone)]
struct AppState {
	client: Client,
	pool: Arc<DriverPool>,
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
	let client = Client::new();
	let pool = Arc::new(DriverPool::from_command(cli.command).await?);
	let state = AppState {
		client: client.clone(),
		pool: Arc::clone(&pool),
	};

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
		.with_state(state);

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

	Arc::into_inner(pool).unwrap().shutdown(client).await?;

	Ok(())
}

#[derive(Deserialize, Serialize)]
struct WebDriverResponse<T> {
	value: T,
}

async fn handle_status(State(state): State<AppState>) -> Result<Response, Response> {
	#[derive(Serialize)]
	pub struct Status {
		pub ready: bool,
		pub message: String,
	}

	match state.pool.deref() {
		DriverPool::Shared { driver, .. } => {
			let response = state
				.client
				.get(driver.url().join("status").unwrap())
				.send()
				.await
				.map_err(IntoStatus::into_response)?;

			Ok(Response::from(response).map(Body::new))
		}
		DriverPool::Single { .. } => Ok(Json(WebDriverResponse {
			value: Status {
				ready: true,
				message: String::new(),
			},
		})
		.into_response()),
	}
}

async fn create_session(State(state): State<AppState>, body: Bytes) -> Result<Response, Response> {
	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	struct NewSessionResponse {
		session_id: String,
	}

	async fn post_session(
		state: &AppState,
		driver: &WebDriver,
		body: Bytes,
	) -> Result<(String, Bytes), Response> {
		let response = state
			.client
			.post(driver.url().join("session").unwrap())
			.body(body)
			.send()
			.await
			.map_err(IntoStatus::into_response)?;

		if !response.status().is_success() {
			return Err(Response::from(response).map(Body::new));
		}

		let response = response.bytes().await.map_err(IntoStatus::into_response)?;
		let WebDriverResponse {
			value: NewSessionResponse { session_id },
		} = serde_json::from_slice(&response).unwrap();

		println!("Created new WebDriver session at {}", driver.url());

		Ok((session_id, response))
	}

	let mut hasher = Xxh3Default::new();
	body.hash(&mut hasher);
	let caps_hash = hasher.digest();

	let response = match state.pool.deref() {
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
				let (session_id, response) = post_session(&state, driver, body).await?;

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
				let (session_id, response) = post_session(&state, &driver, body).await?;

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

async fn delete_session(State(state): State<AppState>, extract::Path(id): extract::Path<String>) {
	match state.pool.deref() {
		DriverPool::Shared {
			available, active, ..
		} => {
			let (caps_hash, response) = active.lock().await.remove(&id).unwrap();
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
			let (caps_hash, driver) = active.lock().await.remove(&id).unwrap();
			available
				.lock()
				.await
				.entry(caps_hash)
				.or_default()
				.push((id, driver));
		}
	}
}

async fn proxy_request(
	State(state): State<AppState>,
	extract::Path((id, path)): extract::Path<(String, String)>,
	method: Method,
	body: Bytes,
) -> Result<Response, StatusCode> {
	let mut url = match state.pool.deref() {
		DriverPool::Shared { driver, .. } => driver.url().clone(),
		DriverPool::Single { active, .. } => {
			active.lock().await.get(&id).unwrap().1.driver.url().clone()
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
		.map_err(IntoStatus::into_status)?;

	Ok(Response::from(response).map(Body::new))
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

	async fn shutdown(self, client: Client) -> Result<()> {
		match self {
			Self::Shared {
				driver,
				available,
				active,
			} => {
				if driver.is_remote() {
					let url = driver.url().join("session/").unwrap();

					for id in available
						.into_inner()
						.into_values()
						.flatten()
						.map(|(id, _)| id)
						.chain(active.into_inner().into_keys())
					{
						let response = client.delete(url.join(&id).unwrap()).send().await.unwrap();

						if !response.status().is_success() {
							println!("Failed to shutdown session {id} at {}:", driver.url());
							println!("\t{}", response.text().await.unwrap());
						}
					}
				}

				driver.shutdown().await?;
			}
			Self::Single {
				available, active, ..
			} => {
				for driver in available
					.into_inner()
					.into_values()
					.flatten()
					.map(|(_, driver)| driver)
					.chain(active.into_inner().into_values().map(|(_, driver)| driver))
				{
					driver.driver.shutdown().await?;
				}
			}
		}

		Ok(())
	}
}

trait IntoStatus: Sized {
	#[track_caller]
	fn into_status(self) -> StatusCode;

	#[track_caller]
	fn into_response(self) -> Response {
		self.into_status().into_response()
	}
}

impl IntoStatus for Error {
	fn into_status(self) -> StatusCode {
		if let Some(status) = self.status() {
			return status;
		}

		panic::panic_any(self);
	}
}
