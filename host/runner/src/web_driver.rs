use std::io;

use anyhow::Result;
use url::Url;

use crate::config::WebDriverLocation;

pub enum WebDriver {
	Local(js_bindgen_shared::WebDriver),
	Remote(Url),
}

impl WebDriver {
	pub async fn run(location: WebDriverLocation) -> Result<Self> {
		match location {
			WebDriverLocation::Local { path, args } => Ok(Self::Local(
				js_bindgen_shared::WebDriver::run(&path, &args).await?,
			)),
			WebDriverLocation::Remote(url) => Ok(Self::Remote(url)),
		}
	}

	pub fn url(&self) -> &Url {
		match self {
			Self::Local(web_driver) => web_driver.url(),
			Self::Remote(url) => url,
		}
	}

	pub async fn output_error(self) {
		if let Self::Local(web_driver) = self {
			web_driver.output_error().await;
		}
	}

	pub async fn shutdown(self) -> io::Result<()> {
		match self {
			Self::Local(web_driver) => web_driver.shutdown().await,
			Self::Remote(_) => Ok(()),
		}
	}
}
