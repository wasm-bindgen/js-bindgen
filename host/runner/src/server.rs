use std::collections::BTreeMap;
use std::io::{self, ErrorKind, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::ops::DerefMut;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use anyhow::{Context, Result, bail};
use axum::body::Body;
use axum::extract::State;
use axum::http::HeaderValue;
use axum::http::header::CONTENT_TYPE;
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use js_bindgen_shared::{AtomicFlag, ReadFile};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::config::WorkerKind;
use crate::runner::{SHARED_JS, SHARED_TERMINAL_JS};

const INDEX_HTML: &str = include_str!("js/index.html");
const BROWSER_JS: &str = include_str!("js/dom/browser.mjs");
const BROWSER_SPAWNER_JS: &str = include_str!("js/dom/browser-spawner.mjs");
const BROWSER_WORKER_JS: &str = include_str!("js/worker/browser-worker.mjs");
const BROWSER_SERVICE_JS: &str = include_str!("js/worker/browser-service.mjs");
const SERVER_JS: &str = include_str!("js/dom/server.mjs");
const SERVER_SPAWNER_JS: &str = include_str!("js/dom/server-spawner.mjs");
const SERVER_DEDICATED_JS: &str = include_str!("js/worker/server-dedicated.mjs");
const SERVER_SHARED_JS: &str = include_str!("js/worker/server-shared.mjs");
const SERVER_SERVICE_JS: &str = include_str!("js/worker/server-service.mjs");
const SHARED_SPAWNER_JS: &str = include_str!("js/dom/shared-spawner.mjs");
const SHARED_BROWSER_JS: &str = include_str!("js/shared/shared-browser.mjs");
const SHARED_SERVER_JS: &str = include_str!("js/dom/shared-server.mjs");
const SHARED_IMPORT_JS: &str = include_str!("js/shared/shared-import.mjs");

pub struct HttpServer {
	url: String,
	server: JoinHandle<Result<(), io::Error>>,
	state: Arc<ServerState>,
}

struct ServerState {
	signals: Signals,
	reports: Mutex<ReportState>,
	wasm_bytes: ReadFile,
	import_js: ReadFile,
	test_data_json: String,
}

#[derive(Default)]
struct Signals {
	shutdown: AtomicFlag,
	status: AtomicU8,
}

#[derive(Default)]
struct ReportState {
	index: usize,
	finished: Option<usize>,
	reports: BTreeMap<usize, Report>,
}

impl HttpServer {
	pub async fn start(
		address: Option<SocketAddr>,
		headless: bool,
		worker: Option<WorkerKind>,
		wasm_bytes: ReadFile,
		imports_path: &Path,
		test_data_json: String,
	) -> Result<Self> {
		let listener = Self::bind_address(address).await?;
		let local_addr = listener.local_addr()?;

		let url = format!(
			"http://{}:{}{}",
			local_addr.ip(),
			local_addr.port(),
			worker.map_or_else(String::new, |worker| format!("/?worker={}", worker as u8))
		);

		let state = Arc::new(ServerState {
			wasm_bytes,
			import_js: ReadFile::new(imports_path)?,
			test_data_json,
			signals: Signals::default(),
			reports: Mutex::new(ReportState::default()),
		});
		let serve = axum::serve(listener, Self::router(Arc::clone(&state), headless, worker));
		let serve = serve.with_graceful_shutdown({
			let state = state.clone();
			async move {
				(&state.signals.shutdown).await;
			}
		});
		let server = tokio::spawn(serve.into_future());

		Ok(Self { url, server, state })
	}

	pub async fn shutdown(self) {
		self.state.signals.shutdown.signal();
		self.server.await.unwrap().unwrap();
	}

	pub async fn wait(self) -> Status {
		self.server.await.unwrap().unwrap();
		let status = self.state.signals.status.load(Ordering::Relaxed);
		Status::from_repr(status).unwrap()
	}

	fn router(state: Arc<ServerState>, headless: bool, worker: Option<WorkerKind>) -> Router {
		let mut router = Router::new()
			.route("/", get(async || response("text/html", INDEX_HTML)))
			.route(
				"/dom/script.mjs",
				get(async move || {
					let file = if headless {
						if worker.is_some() {
							BROWSER_SPAWNER_JS
						} else {
							BROWSER_JS
						}
					} else if worker.is_some() {
						SERVER_SPAWNER_JS
					} else {
						SERVER_JS
					};
					response("application/javascript", file)
				}),
			)
			.route(
				"/shared/shared.mjs",
				get(async || response("application/javascript", SHARED_JS)),
			)
			.route(
				"/shared/shared-import.mjs",
				get(async || response("application/javascript", SHARED_IMPORT_JS)),
			)
			.route(
				"/test-data.json",
				get(async |State(state): State<Arc<ServerState>>| {
					response("application/json", state.test_data_json.clone())
				}),
			)
			.route(
				"/wasm.wasm",
				get(async |State(state): State<Arc<ServerState>>| -> Response {
					response("application/wasm", state.wasm_bytes.to_owned())
				}),
			)
			.route(
				"/imports.mjs",
				get(async |State(state): State<Arc<ServerState>>| {
					response("application/javascript", state.import_js.to_owned())
				}),
			);

		if worker.is_some() {
			router = router.route(
				"/dom/shared-spawner.mjs",
				get(async || response("application/javascript", SHARED_SPAWNER_JS)),
			);
		}

		if headless {
			match worker {
				Some(WorkerKind::Dedicated | WorkerKind::Shared) => {
					router = router.route(
						"/worker/script.mjs",
						get(async || response("application/javascript", BROWSER_WORKER_JS)),
					);
				}
				Some(WorkerKind::Service) => {
					router = router.route(
						"/worker/script.mjs",
						get(async || response("application/javascript", BROWSER_SERVICE_JS)),
					);
				}
				None => (),
			}

			router = router
				.route(
					"/shared/shared-browser.mjs",
					get(async || response("application/javascript", SHARED_BROWSER_JS)),
				)
				.route(
					"/shared/shared-terminal.mjs",
					get(async || response("application/javascript", SHARED_TERMINAL_JS)),
				)
				.route(
					"/report",
					post(
						async |State(state): State<Arc<ServerState>>,
						       Json(message): Json<ReportMessage>| {
							let mut report_state = state.reports.lock().await;
							let ReportState {
								index,
								finished,
								reports,
							} = report_state.deref_mut();

							if message.order - *index == 0 {
								*index += 1;
								message.report.emit();

								while let Some(report) = reports.remove(index) {
									*index += 1;
									report.emit();
								}

								if let Some(messages) = finished
									&& messages == index
								{
									state.signals.shutdown.signal();
								}
							} else {
								report_state.reports.insert(message.order, message.report);
							}
						},
					),
				)
				.route(
					"/finished",
					post(
						async |State(state): State<Arc<ServerState>>,
						       Json(finished): Json<Finished>| {
							state
								.signals
								.status
								.store(finished.status.to_repr(), Ordering::Relaxed);

							let mut report_state = state.reports.lock().await;

							if finished.messages == 0 {
								while let Some((_, report)) = report_state.reports.pop_first() {
									report.emit();
								}

								state.signals.shutdown.signal();
							} else if report_state.index == finished.messages {
								state.signals.shutdown.signal();
							} else {
								report_state.finished = Some(finished.messages);
							}
						},
					),
				);
		} else {
			if let Some(worker) = worker {
				router = router.route(
					"/worker/script.mjs",
					get(async move || {
						response(
							"application/javascript",
							match worker {
								WorkerKind::Dedicated => SERVER_DEDICATED_JS,
								WorkerKind::Shared => SERVER_SHARED_JS,
								WorkerKind::Service => SERVER_SERVICE_JS,
							},
						)
					}),
				);
			}

			router = router.route(
				"/dom/shared-server.mjs",
				get(async || response("application/javascript", SHARED_SERVER_JS)),
			);
		}

		router.with_state(state)
	}

	async fn bind_address(address: Option<SocketAddr>) -> Result<TcpListener> {
		let default_addr =
			address.unwrap_or_else(|| SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000));
		match TcpListener::bind(default_addr).await {
			Ok(listener) => Ok(listener),
			Err(err) if matches!(err.kind(), ErrorKind::AddrInUse) && address.is_none() => {
				TcpListener::bind(SocketAddr::new(default_addr.ip(), 0))
					.await
					.context("failed to bind default address")
			}
			Err(err) => Err(err).context("failed to bind address"),
		}
	}

	pub fn url(&self) -> &str {
		&self.url
	}
}

#[derive(Deserialize)]
struct ReportMessage {
	order: usize,
	report: Report,
}

#[derive(Deserialize)]
struct Report {
	stream: Stream,
	line: String,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum Stream {
	Stdout,
	Stderr,
}

impl Report {
	fn emit(&self) {
		match self.stream {
			Stream::Stdout => {
				print!("{}", self.line);
				io::stdout().flush().unwrap();
			}
			Stream::Stderr => {
				eprint!("{}", self.line);
				io::stderr().flush().unwrap();
			}
		}
	}
}

#[derive(Clone, Copy, Deserialize)]
struct Finished {
	status: Status,
	messages: usize,
}

#[derive(Clone, Copy, Deserialize_repr)]
#[repr(u8)]
pub enum Status {
	Ok,
	Failed,
	Abnormal,
}

impl Status {
	fn to_repr(self) -> u8 {
		match self {
			Self::Ok => 0,
			Self::Failed => 1,
			Self::Abnormal => 2,
		}
	}

	#[track_caller]
	fn from_repr(value: u8) -> Result<Self> {
		match value {
			0 => Ok(Self::Ok),
			1 => Ok(Self::Failed),
			2 => Ok(Self::Abnormal),
			_ => bail!("unexpected value for `Status`: {value}"),
		}
	}
}

fn response(content_type: &'static str, body: impl Into<Body>) -> Response {
	let mut response = Response::new(body.into());
	response
		.headers_mut()
		.insert(CONTENT_TYPE, HeaderValue::from_static(content_type));

	let headers = response.headers_mut();
	headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
	headers.insert(
		"Access-Control-Allow-Methods",
		HeaderValue::from_static("GET, POST, OPTIONS"),
	);
	headers.insert(
		"Access-Control-Allow-Headers",
		HeaderValue::from_static("Content-Type"),
	);
	headers.insert(
		"Cross-Origin-Opener-Policy",
		HeaderValue::from_static("same-origin"),
	);
	headers.insert(
		"Cross-Origin-Embedder-Policy",
		HeaderValue::from_static("require-corp"),
	);

	response
}
