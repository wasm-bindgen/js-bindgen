use std::fs;
use std::io::ErrorKind;
use std::iter::Copied;
use std::path::Path;
use std::process::Command;
use std::slice::Iter;
use std::sync::LazyLock;
use std::time::Instant;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Args, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use super::{HostTarget, HostTargets, metadata};
use crate::check::CheckTool;
use crate::command::{self, CargoCommand};

#[derive(Args)]
pub struct Check {
	#[arg(long, value_delimiter = ',', default_value = Tools::default_arg())]
	tools: Vec<Tools>,
	#[arg(long, short, default_value = HostTargets::default_arg())]
	targets: Vec<HostTargets>,
}

enum_with_all!(pub enum Tools, Tool(Tool), "tools");

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Tool {
	Shared(CheckTool),
	Host(HostTool),
}

#[derive(Clone, Copy, EnumIter, Eq, PartialEq, ValueEnum)]
pub enum HostTool {
	NpmAudit,
	Tsc,
	EsLint,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			tools: vec![Tools::default()],
			targets: vec![HostTargets::default()],
		}
	}
}

impl Check {
	pub fn new(tools: Vec<Tools>, targets: Vec<HostTargets>) -> Self {
		Self { tools, targets }
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let tools = Tool::from_tools(self.tools)?;
		let start = Instant::now();

		for tool in tools {
			match tool {
				Tool::Shared(CheckTool::Clippy) => {
					let commands = [
						CargoCommand {
							title: "Check",
							sub_command: "clippy",
							args: &["--", "-D", "warnings"],
							envs: &[],
						},
						CargoCommand {
							title: "Check Tests",
							sub_command: "clippy",
							args: &["--tests", "--benches", "--examples", "--", "-D", "warnings"],
							envs: &[],
						},
						CargoCommand {
							title: "Check Doc",
							sub_command: "doc",
							args: &["--no-deps", "--document-private-items"],
							envs: &[("RUSTDOCFLAGS", "-D warnings")],
						},
					];
					let targets = HostTarget::from_targets(self.targets.clone())?;
					metadata::run(&commands, &targets, true, verbose)?;
				}
				Tool::Shared(CheckTool::RustSec) => {
					let mut command = Command::new("cargo");
					command.arg("audit");
					command::run("RustSec", command, verbose)?;
				}
				Tool::Host(HostTool::NpmAudit) => {
					Self::npm_lock_file("js-bindgen-ld", "ld/src/js", verbose)?;
					Self::npm_lock_file("js-bindgen-runner", "runner/src/js", verbose)?;
				}
				Tool::Host(HostTool::Tsc) => {
					Self::tsc("js-bindgen-ld", "ld/src/js", verbose)?;
					Self::tsc("js-bindgen-runner", "runner/src/js", verbose)?;
				}
				Tool::Host(HostTool::EsLint) => {
					Self::eslint("js-bindgen-ld", "ld/src/js", verbose)?;
					Self::eslint("js-bindgen-runner", "runner/src/js", verbose)?;
				}
				Tool::Shared(CheckTool::Tombi) => {
					let mut command = Command::new("tombi");
					command.args(["lint", "--error-on-warnings", "."]);
					command::run("Tombi Lint", command, verbose)?;
				}
				Tool::Shared(CheckTool::CargoSpellcheck) => {
					let mut command = Command::new("cargo");
					command.args(["spellcheck", "-m", "1"]);
					command::run("`cargo-spellcheck`", command, verbose)?;
				}
				Tool::Shared(CheckTool::Typos) => {
					command::run("Typos", Command::new("typos"), verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}

	fn tsc(package: &str, path: &str, verbose: bool) -> Result<()> {
		Self::npm_install(package, path, verbose)?;

		let mut command = Command::new("tsc");
		command.current_dir(path).arg("-b").arg("--noEmit");
		command::run(&format!("TSC `{package}`"), command, verbose)?;

		Ok(())
	}

	fn eslint(package: &str, path: &str, verbose: bool) -> Result<()> {
		Self::npm_install(package, path, verbose)?;

		let mut command = Command::new("npx");
		command.current_dir(path).arg("eslint");
		command::run(&format!("ESLint `{package}`"), command, verbose)?;

		Ok(())
	}

	fn npm_lock_file(package: &str, path: &str, verbose: bool) -> Result<()> {
		let needs_install = match fs::metadata(Path::new(path).join("package-lock.json")) {
			Ok(meta) => {
				let lock_mtime = meta.modified()?;
				let pkg_mtime = fs::metadata(Path::new(path).join("package.json"))?.modified()?;

				lock_mtime < pkg_mtime
			}
			Err(error) if error.kind() == ErrorKind::NotFound => true,
			Err(error) => return Err(error.into()),
		};

		if needs_install {
			let mut command = Command::new("npm");
			command
				.current_dir(path)
				.arg("install")
				.arg("--package-lock-only")
				.arg("--no-audit")
				.arg("--no-fund");

			command::run(&format!("NPM Install `{package}`"), command, verbose)?;
		}

		let mut command = Command::new("npm");
		command.current_dir(path).arg("audit");
		command::run(&format!("NPM Audit `{package}`"), command, verbose)?;

		Ok(())
	}

	fn npm_install(package: &str, path: &str, verbose: bool) -> Result<()> {
		let needs_install = match fs::metadata(Path::new(path).join("package-lock.json")) {
			Ok(meta) => 'outer: {
				let lock_mtime = meta.modified()?;
				let pkg_mtime = fs::metadata(Path::new(path).join("package.json"))?.modified()?;

				if lock_mtime < pkg_mtime {
					break 'outer true;
				}

				match fs::metadata(Path::new(path).join("node_modules/.package-lock.json")) {
					Ok(meta) => meta.modified()? < pkg_mtime,
					Err(error) if error.kind() == ErrorKind::NotFound => true,
					Err(error) => return Err(error.into()),
				}
			}
			Err(error) if error.kind() == ErrorKind::NotFound => true,
			Err(error) => return Err(error.into()),
		};

		if needs_install {
			let mut command = Command::new("npm");
			command
				.current_dir(path)
				.arg("install")
				.arg("--no-audit")
				.arg("--no-fund");

			command::run(&format!("NPM Install `{package}`"), command, verbose)?;
		}

		Ok(())
	}
}

impl Default for Tool {
	fn default() -> Self {
		Self::Shared(CheckTool::default())
	}
}

impl IntoEnumIterator for Tool {
	type Iterator = Copied<Iter<'static, Self>>;

	fn iter() -> Self::Iterator {
		Self::value_variants().iter().copied()
	}
}

impl ValueEnum for Tool {
	fn value_variants<'a>() -> &'a [Self] {
		static VALUES: LazyLock<Vec<Tool>> = LazyLock::new(|| {
			CheckTool::iter()
				.map(Tool::Shared)
				.chain(HostTool::iter().map(Tool::Host))
				.collect()
		});

		&VALUES
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::Shared(tool) => tool.to_possible_value(),
			Self::Host(tool) => tool.to_possible_value(),
		}
	}
}
