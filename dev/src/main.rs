mod build;
mod command;
mod permutation;
mod process;
mod test;

use std::fmt;
use std::fmt::{Display, Formatter};

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Parser, Subcommand, ValueEnum};
use strum::{EnumCount, EnumIter, IntoEnumIterator, VariantArray};

use crate::build::Build;
use crate::permutation::Toolchain;
use crate::test::Test;

#[derive(Parser)]
struct Cli {
	#[arg(short, long, global = true)]
	verbose: bool,
	#[command(subcommand)]
	command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
	Build(Build),
	Test(Test),
}

#[derive(Clone, Copy, ValueEnum, VariantArray)]
enum Target {
	Wasm32,
	Wasm64,
}

#[derive(Clone, Copy, ValueEnum, VariantArray)]
enum TargetFeature {
	Default,
	Atomics,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Runner {
	Engine(Engine),
	WebDriver(WebDriver),
}

fn main() -> Result<()> {
	Cli::parse().execute()
}

impl Cli {
	fn execute(self) -> Result<()> {
		match self.command {
			CliCommand::Build(build) => build.execute(self.verbose),
			CliCommand::Test(test) => test.execute(self.verbose),
		}
	}
}

impl Target {
	fn rustflags_env(self) -> &'static str {
		match self {
			Self::Wasm32 => "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS",
			Self::Wasm64 => "CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUSTFLAGS",
		}
	}

	fn rustdocflags_env(self) -> &'static str {
		match self {
			Self::Wasm32 => "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTDOCFLAGS",
			Self::Wasm64 => "CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUSTDOCFLAGS",
		}
	}

	fn toolchain(self, target_feature: TargetFeature) -> Toolchain {
		match (self, target_feature) {
			(Self::Wasm64, _) | (_, TargetFeature::Atomics) => Toolchain::Nightly,
			(Self::Wasm32, TargetFeature::Default) => Toolchain::Any,
		}
	}

	fn args(self, target_feature: TargetFeature) -> &'static [&'static str] {
		match (self, target_feature) {
			(Self::Wasm32, TargetFeature::Default) => &["--target", "wasm32-unknown-unknown"],
			(Self::Wasm32, TargetFeature::Atomics) => &[
				"--target",
				"wasm32-unknown-unknown",
				"-Zbuild-std=panic_abort,std",
			],
			(Self::Wasm64, _) => &[
				"--target",
				"wasm64-unknown-unknown",
				"-Zbuild-std=panic_abort,std",
			],
		}
	}
}

impl Display for Target {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Wasm32 => f.write_str("Wasm32"),
			Self::Wasm64 => f.write_str("Wasm64"),
		}
	}
}

impl TargetFeature {
	fn flags(self) -> Option<&'static str> {
		match self {
			Self::Default => None,
			Self::Atomics => Some("-Ctarget-feature=+atomics"),
		}
	}

	fn supports_atomics(self) -> bool {
		match self {
			Self::Default => false,
			Self::Atomics => true,
		}
	}
}

impl Display for TargetFeature {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Default => Ok(()),
			Self::Atomics => f.write_str("Atomics"),
		}
	}
}

impl Runner {
	fn iter() -> impl Iterator<Item = Self> {
		Engine::iter()
			.map(Self::Engine)
			.chain(WebDriver::iter().map(Self::WebDriver))
	}

	fn env(self) -> &'static str {
		match self {
			Self::Engine(engine) => engine.env(),
			Self::WebDriver(web_driver) => web_driver.runner_env(),
		}
	}

	fn supports_target(self, target: Target) -> bool {
		match target {
			Target::Wasm32 => true,
			Target::Wasm64 => match self {
				Self::Engine(_) | Self::WebDriver(WebDriver::Chrome | WebDriver::Gecko) => true,
				#[cfg(target_os = "macos")]
				Self::WebDriver(WebDriver::Safari) => false,
			},
		}
	}

	fn supports_rab(self) -> bool {
		match self {
			Self::Engine(_) => true,
			Self::WebDriver(WebDriver::Chrome | WebDriver::Gecko) => false,
			#[cfg(target_os = "macos")]
			Self::WebDriver(WebDriver::Safari) => false,
		}
	}

	fn supports_sab(self) -> bool {
		match self {
			Self::Engine(_) => true,
			Self::WebDriver(WebDriver::Chrome | WebDriver::Gecko) => false,
			#[cfg(target_os = "macos")]
			Self::WebDriver(WebDriver::Safari) => true,
		}
	}
}

impl Display for Runner {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Engine(engine) => engine.fmt(f),
			Self::WebDriver(web_driver) => web_driver.fmt(f),
		}
	}
}

impl ValueEnum for Runner {
	fn value_variants<'a>() -> &'a [Self] {
		const COUNT: usize = Engine::COUNT + WebDriver::COUNT;
		const ARRAY: [Runner; COUNT] = {
			let mut array = [Runner::Engine(Engine::Deno); COUNT];
			let mut index = 0;

			while index < Engine::COUNT {
				array[index] = Runner::Engine(Engine::VARIANTS[index]);
				index += 1;
			}

			while index < Engine::COUNT + WebDriver::COUNT {
				array[index] = Runner::WebDriver(WebDriver::VARIANTS[index - Engine::COUNT]);
				index += 1;
			}

			array
		};

		&ARRAY
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		Some(PossibleValue::new(self.env()))
	}
}

#[derive(Clone, Copy, EnumCount, EnumIter, Eq, PartialEq, VariantArray)]
enum Engine {
	Deno,
	NodeJs,
}

impl Engine {
	fn env(self) -> &'static str {
		match self {
			Self::Deno => "deno",
			Self::NodeJs => "node-js",
		}
	}
}

impl Display for Engine {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Deno => "Deno",
			Self::NodeJs => "Node.js",
		};

		f.write_str(name)
	}
}

#[derive(Clone, Copy, EnumCount, EnumIter, Eq, PartialEq, VariantArray)]
enum WebDriver {
	Chrome,
	Gecko,
	#[cfg(target_os = "macos")]
	Safari,
}

impl WebDriver {
	fn runner_env(self) -> &'static str {
		match self {
			Self::Chrome => "chrome-driver",
			Self::Gecko => "gecko-driver",
			#[cfg(target_os = "macos")]
			Self::Safari => "safari-driver",
		}
	}

	fn remote_env(self) -> &'static str {
		match self {
			Self::Chrome => "JBG_TEST_CHROME_DRIVER_REMOTE",
			Self::Gecko => "JBG_TEST_GECKO_DRIVER_REMOTE",
			#[cfg(target_os = "macos")]
			Self::Safari => "JBG_TEST_SAFARI_DRIVER_REMOTE",
		}
	}

	fn remote_url(self) -> &'static str {
		match self {
			Self::Chrome => "http://127.0.0.1:8000",
			Self::Gecko => "http://127.0.0.1:8001",
			#[cfg(target_os = "macos")]
			Self::Safari => "http://127.0.0.1:8002",
		}
	}

	fn short_name(self) -> &'static str {
		match self {
			Self::Chrome => "chrome",
			Self::Gecko => "gecko",
			#[cfg(target_os = "macos")]
			Self::Safari => "safari",
		}
	}

	fn port(self) -> &'static str {
		match self {
			Self::Chrome => "8000",
			Self::Gecko => "8001",
			#[cfg(target_os = "macos")]
			Self::Safari => "8002",
		}
	}
}

impl Display for WebDriver {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Chrome => "Chrome",
			Self::Gecko => "Firefox",
			#[cfg(target_os = "macos")]
			Self::Safari => "Safari",
		};

		f.write_str(name)
	}
}
