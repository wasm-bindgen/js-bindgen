mod check;
mod fmt;
mod metadata;
mod permutation;
mod process;
mod test;
mod util;

use std::fmt::{Display, Formatter};

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};
use strum::EnumIter;

use self::check::{Check, Tools};
pub use self::check::{ClientTool, Tool};
use self::fmt::Fmt;
use self::permutation::Toolchain;
use self::test::Test;
use self::util::ToolchainParser;
use crate::FmtTools;
use crate::command::CargoCommand;

#[derive(Subcommand)]
pub enum Client {
	All {
		#[arg(long, value_delimiter = ',', default_value = FmtTools::default_arg())]
		fmt_tools: Vec<FmtTools>,
		#[arg(long, value_delimiter = ',', default_value = Tools::default_arg())]
		check_tools: Vec<Tools>,
		#[command(flatten)]
		test: Test,
	},
	Fmt(Fmt),
	Build {
		#[command(flatten)]
		args: ClientArgs,
	},
	Check(Check),
	Test(Test),
}

#[derive(Args, Clone)]
pub struct ClientArgs {
	#[arg(long, value_delimiter = ',', default_value = Targets::default_arg())]
	targets: Vec<Targets>,
	#[arg(long, value_delimiter = ',', default_value = TargetFeatures::default_arg())]
	target_features: Vec<TargetFeatures>,
	#[arg(long, value_parser = ToolchainParser, default_value = "nightly")]
	nightly_toolchain: String,
}

impl Client {
	pub fn fmt() -> Self {
		Self::Fmt(Fmt::default())
	}

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

	pub fn check(tools: Vec<Tool>, all: bool) -> Self {
		let client_args = if all {
			ClientArgs::all()
		} else {
			ClientArgs::default()
		};
		let tools = tools.into_iter().map(Tools::Tool).collect();

		Self::Check(Check::new(client_args, tools))
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
			Self::All {
				fmt_tools,
				check_tools,
				test,
			} => {
				Self::Fmt(Fmt::new(fmt_tools)).execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Build {
					args: test.args().clone(),
				}
				.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Check(Check::new(test.args().clone(), check_tools)).execute(verbose)?;
				println!("-------------------------");
				println!();
				test.execute(verbose)?;

				Ok(())
			}
			Self::Fmt(fmt) => fmt.execute(verbose),
			Self::Build { args } => {
				let command = CargoCommand {
					title: "Build",
					sub_command: "build",
					args: &[],
					envs: &[],
				};
				let duration = metadata::run(args, &[command], verbose)?;

				println!("-------------------------");
				println!("Total Time: {:.2}s", duration.as_secs_f32());

				Ok(())
			}
			Self::Check(check) => check.execute(verbose),
			Self::Test(test) => test.execute(verbose),
		}
	}
}

impl Default for ClientArgs {
	fn default() -> Self {
		Self {
			targets: vec![Targets::default()],
			target_features: vec![TargetFeatures::default()],
			nightly_toolchain: String::from("nightly"),
		}
	}
}

impl ClientArgs {
	fn all() -> Self {
		Self {
			targets: Targets::all(),
			target_features: TargetFeatures::all(),
			nightly_toolchain: String::from("nightly"),
		}
	}
}

enum_with_all!(enum Targets, Target(Target), "targets");

#[derive(Clone, Copy, Default, EnumIter, Eq, PartialEq, ValueEnum)]
enum Target {
	#[default]
	Wasm32,
	Wasm64,
}

enum_with_all!(enum TargetFeatures, TargetFeature(TargetFeature), "target-features");

#[derive(Clone, Copy, Default, EnumIter, Eq, PartialEq, ValueEnum)]
enum TargetFeature {
	#[default]
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
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Default => Ok(()),
			Self::Atomics => f.write_str("Atomics"),
		}
	}
}
