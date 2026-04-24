mod metadata;
mod permutation;
mod process;
mod test;
mod util;

use std::fmt;
use std::fmt::{Display, Formatter};

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use self::permutation::Toolchain;
use self::test::Test;
use self::util::ToolchainParser;
use crate::command::RunCommand;

#[derive(Subcommand)]
pub enum Client {
	All(Test),
	Build {
		#[command(flatten)]
		args: ClientArgs,
	},
	Check {
		#[command(flatten)]
		args: ClientArgs,
	},
	Test(Test),
}

#[derive(Args, Clone)]
pub struct ClientArgs {
	#[arg(long, value_delimiter = ',', default_value = "wasm32")]
	targets: Vec<Target>,
	#[arg(long, value_delimiter = ',', default_value = "default")]
	target_features: Vec<TargetFeature>,
	#[arg(long, value_parser = ToolchainParser, default_value = "nightly")]
	nightly_toolchain: String,
}

impl Client {
	pub fn build(all: bool) -> Self {
		if all {
			Self::Build {
				args: ClientArgs::all(),
			}
		} else {
			Self::Build {
				args: ClientArgs::default(),
			}
		}
	}

	pub fn check(all: bool) -> Self {
		if all {
			Self::Check {
				args: ClientArgs::all(),
			}
		} else {
			Self::Check {
				args: ClientArgs::default(),
			}
		}
	}

	pub fn test(all: bool) -> Self {
		if all {
			Self::Test(Test::all())
		} else {
			Self::Test(Test::default())
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All(test) => {
				Self::Build {
					args: test.args().clone(),
				}
				.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Check {
					args: test.args().clone(),
				}
				.execute(verbose)?;
				println!("-------------------------");
				println!();
				test.execute(verbose)?;

				Ok(())
			}
			Self::Build { args } => {
				let command = RunCommand {
					title: "Build",
					sub_command: "build",
					args: &[],
					envs: &[],
				};
				metadata::run(&args, &[command], verbose)
			}
			Self::Check { args } => {
				let commands = [
					RunCommand {
						title: "Check",
						sub_command: "clippy",
						args: &["--keep-going", "--", "-D", "warnings"],
						envs: &[],
					},
					RunCommand {
						title: "Check Tests",
						sub_command: "clippy",
						args: &[
							"--keep-going",
							"--tests",
							"--benches",
							"--",
							"-D",
							"warnings",
						],
						envs: &[],
					},
					RunCommand {
						title: "Doc",
						sub_command: "doc",
						args: &["--keep-going", "--no-deps", "--document-private-items"],
						envs: &[("RUSTDOCFLAGS", "-D warnings")],
					},
				];
				metadata::run(&args, &commands, verbose)
			}
			Self::Test(test) => test.execute(verbose),
		}
	}
}

impl Default for ClientArgs {
	fn default() -> Self {
		Self {
			targets: vec![Target::Wasm32],
			target_features: vec![TargetFeature::Default],
			nightly_toolchain: String::from("nightly"),
		}
	}
}

impl ClientArgs {
	fn all() -> Self {
		Self {
			targets: Target::iter().collect(),
			target_features: TargetFeature::iter().collect(),
			nightly_toolchain: String::from("nightly"),
		}
	}
}

#[derive(Clone, Copy, EnumIter, ValueEnum)]
enum Target {
	Wasm32,
	Wasm64,
}

#[derive(Clone, Copy, EnumIter, ValueEnum)]
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
