use std::ops::Deref;
use std::panic;

use anyhow::{Result, bail};
use axum::Json;
use axum::body::{Body, Bytes};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use js_bindgen_shared::WebDriver;
use reqwest::Error;
use serde::{Deserialize, Serialize};

pub struct Client(reqwest::Client);

impl Client {
	pub fn new() -> Self {
		Self(reqwest::Client::new())
	}

	pub async fn status(&self, driver: &WebDriver) -> Result<Response, Response> {
		let response = self
			.0
			.get(driver.url().join("status").unwrap())
			.send()
			.await
			.map_err(IntoAxum::into_response)?;

		Ok(Response::from(response).map(Body::new))
	}

	pub async fn create_session(
		&self,
		driver: &WebDriver,
		body: Bytes,
	) -> Result<(String, Bytes), Response> {
		let response = self
			.0
			.post(driver.url().join("session").unwrap())
			.body(body)
			.send()
			.await
			.map_err(IntoAxum::into_response)?;

		if !response.status().is_success() {
			return Err(Response::from(response).map(Body::new));
		}

		let response = response.bytes().await.map_err(IntoAxum::into_response)?;
		let WebDriverResponse {
			value: NewSessionResponse { session_id },
		} = serde_json::from_slice(&response).unwrap();

		Ok((session_id, response))
	}

	pub async fn delete_session(&self, driver: &WebDriver, id: &str) -> Result<()> {
		let mut url = driver.url().clone();
		let mut segments = url.path_segments_mut().unwrap();
		segments.push("session");
		segments.push(id);
		drop(segments);

		let response = self.0.delete(url).send().await.unwrap();

		if !response.status().is_success() {
			let response = response
				.text()
				.await
				.unwrap_or_else(|error| error.to_string());

			bail!(
				"Failed to shutdown session {id} at {}:\n\t{response}",
				driver.url(),
			);
		}

		Ok(())
	}

	pub async fn navigate_to(
		&self,
		driver: &WebDriver,
		id: &str,
		url: &str,
	) -> Result<(), Response> {
		let mut driver_url = driver.url().clone();
		let mut segments = driver_url.path_segments_mut().unwrap();
		segments.push("session");
		segments.push(id);
		segments.push("url");
		drop(segments);

		self.0
			.post(driver_url)
			.json(&NavigateToRequest { url })
			.send()
			.await
			.map_err(IntoAxum::into_response)?;

		Ok(())
	}
}

impl Deref for Client {
	type Target = reqwest::Client;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Deserialize, Serialize)]
pub struct WebDriverResponse<T> {
	pub value: T,
}

#[derive(Serialize)]
pub struct StatusResponse {
	pub ready: bool,
	pub message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewSessionResponse {
	session_id: String,
}

#[derive(Serialize)]
struct NavigateToRequest<'url> {
	url: &'url str,
}

#[derive(Serialize)]
struct ErrorResponse {
	error: WebDriverErrorKind,
	message: String,
	stacktrace: &'static str,
}

#[derive(Clone, Copy, Serialize)]
pub enum WebDriverErrorKind {
	#[serde(rename = "invalid session id")]
	InvalidSessionId,
}

impl WebDriverErrorKind {
	pub fn to_response(self, message: String) -> Response {
		let mut response = Json(WebDriverResponse {
			value: ErrorResponse {
				error: self,
				message,
				stacktrace: "",
			},
		})
		.into_response();
		*response.status_mut() = self.status_code();

		response
	}

	fn status_code(self) -> StatusCode {
		match self {
			Self::InvalidSessionId => StatusCode::NOT_FOUND,
		}
	}
}

pub trait IntoAxum: Sized {
	#[track_caller]
	fn into_response(self) -> Response;
}

impl IntoAxum for Error {
	fn into_response(self) -> Response {
		if let Some(status) = self.status() {
			return status.into_response();
		}

		panic::panic_any(self);
	}
}
