use std::any::Any;
use std::borrow::Cow;
use std::ffi::OsString;
use std::hash::Hash;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::{ControlFlow, Deref};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, panic};

use anyhow::{Error, Result};
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::Response;
use axum::routing::{any, delete, post};
use axum::{Router, extract};
use clap::Parser;
use hashbrown::HashMap;
use js_bindgen_shared::{AtomicFlag, WebDriver, WebDriverKind};
use mime::APPLICATION_JSON;
use reqwest::{Client, Request};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::catch_panic::CatchPanicLayer;
use xxhash_rust::xxh3::Xxh3Default;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
	driver: WebDriverKind,
	#[arg(short, long)]
	port: Option<u16>,
	path: Option<PathBuf>,
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
	let Args { driver, port, path } = Args::parse();

	let path = if let Some(path) = path {
		Cow::Owned(path)
	} else if let Some(path) = env::var_os(format!("JBG_TEST_{}_PATH", driver.to_env())) {
		Cow::Owned(path.into())
	} else {
		Cow::Borrowed(Path::new(driver.to_binary()))
	};

	let args = js_bindgen_shared::env_args(&format!("JBG_TEST_{}_ARGS", driver.to_env()))?;

	let listener = TcpListener::bind(SocketAddrV4::new(
		Ipv4Addr::LOCALHOST,
		port.unwrap_or_default(),
	))
	.await?;

	println!("Server listening on: {}", listener.local_addr()?);

	let shutdown = Arc::new(AtomicFlag::new());
	let pool = if driver.multi_session_support() {
		AppState {
			client: Client::new(),
			pool: Arc::new(DriverPool::Shared {
				driver: WebDriver::run(&path, &args).await?,
				available: Mutex::new(HashMap::new()),
				active: Mutex::new(HashMap::new()),
			}),
		}
	} else {
		AppState {
			client: Client::new(),
			pool: Arc::new(DriverPool::Single {
				path,
				args,
				available: Mutex::new(HashMap::new()),
				active: Mutex::new(HashMap::new()),
			}),
		}
	};

	let app = Router::new()
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
		.with_state(pool);

	axum::serve(listener, app)
		.with_graceful_shutdown(async move {
			shutdown.deref().await;
		})
		.await
		.map_err(Error::from)
}

#[derive(Deserialize)]
struct WebDriverResponse {
	value: NewSessionResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewSessionResponse {
	session_id: String,
}

async fn create_session(
	State(state): State<AppState>,
	body: Bytes,
) -> Result<Response, StatusCode> {
	async fn post_session(
		state: &AppState,
		driver: &WebDriver,
		body: Bytes,
	) -> Result<ControlFlow<Response, (String, Bytes)>, StatusCode> {
		let response = state
			.client
			.post(driver.url().join("session").unwrap())
			.body(body)
			.send()
			.await
			.map_err(IntoStatus::into_status)?;

		if !response.status().is_success() {
			return Ok(ControlFlow::Break(Response::from(response).map(Body::new)));
		}

		let response = response.bytes().await.map_err(IntoStatus::into_status)?;
		let WebDriverResponse {
			value: NewSessionResponse { session_id },
		} = serde_json::from_slice(&response).unwrap();

		println!("Created new WebDriver session at {}", driver.url());

		Ok(ControlFlow::Continue((session_id, response)))
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
				let (session_id, response) = match post_session(&state, driver, body).await? {
					ControlFlow::Continue(result) => result,
					ControlFlow::Break(response) => return Ok(response),
				};

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
				let driver = WebDriver::run(path, args).await.unwrap();

				let (session_id, response) = match post_session(&state, &driver, body).await? {
					ControlFlow::Continue(result) => result,
					ControlFlow::Break(response) => return Ok(response),
				};

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
) -> Result<Response<Body>, StatusCode> {
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

trait IntoStatus {
	fn into_status(self) -> StatusCode;
}

impl IntoStatus for reqwest::Error {
	fn into_status(self) -> StatusCode {
		if let Some(status) = self.status() {
			return status;
		}

		panic::panic_any(self);
	}
}
