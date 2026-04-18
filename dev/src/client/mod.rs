mod build;
mod permutation;
mod process;
mod test;
mod util;

use std::fmt;
use std::fmt::{Display, Formatter};

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};
use strum::VariantArray;

use self::build::Build;
use self::permutation::Toolchain;
use self::test::Test;
use self::util::ToolchainParser;

#[derive(Subcommand)]
pub enum Client {
	All(Test),
	Build(Build),
	Test(Test),
}

#[derive(Args, Clone, Default)]
pub struct ClientArgs {
	#[arg(long)]
	target: Option<Target>,
	#[arg(long, value_delimiter = ',')]
	target_feature: Vec<TargetFeature>,
	#[arg(long, value_parser = ToolchainParser)]
	nightly_toolchain: Option<String>,
}

impl Client {
	pub fn build() -> Self {
		Self::Build(Build::default())
	}

	pub fn test() -> Self {
		Self::Test(Test::default())
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All(test) => {
				Build::new(test.args().clone()).execute(verbose)?;
				println!("-------------------------");
				println!();
				test.execute(verbose)?;

				Ok(())
			}
			Self::Build(build) => build.execute(verbose),
			Self::Test(test) => test.execute(verbose),
		}
	}
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
