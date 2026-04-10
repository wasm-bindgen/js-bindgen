use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::process::{Command, ExitStatus};
use std::time::{Duration, Instant};
use std::{env, io};

use anstyle::{AnsiColor, Style};
use anyhow::Result;
use clap::builder::{StringValueParser, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Error};

use crate::permutation::{Permutation, Toolchain};

#[must_use = "must only be dropped after operation is finished"]
pub struct Group(Option<(String, Instant)>);

impl Group {
	pub fn announce(text: Cow<'_, str>, verbose: bool) -> Result<Self> {
		let gh_actions = env::var_os("GITHUB_ACTIONS").is_some_and(|value| value == "true");

		if verbose {
			if gh_actions {
				println!("::group::{text}");
			} else {
				println!();
				println!("-------------------------");
				println!("{text}");
				println!("-------------------------");
				println!();
			}
		} else {
			print!("{text} ...");
			io::stdout().flush()?;
		}

		Ok(Self(
			(verbose && gh_actions).then(|| (text.into_owned(), Instant::now())),
		))
	}
}

impl Drop for Group {
	fn drop(&mut self) {
		if let Some((name, start)) = self.0.take() {
			println!("-------------------------");
			println!("Finished {name}: {:.2}s", start.elapsed().as_secs_f32());
			println!("::endgroup::");
		}
	}
}

pub fn cargo(
	permutation: &Permutation,
	nightly_toolchain: Option<&str>,
	subcommand: &str,
) -> Command {
	let mut command = Command::new("cargo");
	command
		.current_dir("../client")
		.envs(permutation.envs())
		.env("CI", "true");

	if let Toolchain::Nightly = permutation.toolchain() {
		if let Some(toolchain) = nightly_toolchain {
			command.arg(format!("+{toolchain}"));
		} else {
			command.arg("+nightly");
		}
	}

	command.arg(subcommand).args(permutation.args());

	command
}

pub fn print_info(command: &Command) {
	let envs = command.get_envs();

	if envs.len() != 0 {
		println!("Running Cargo with environment variables:");

		for (key, value) in envs {
			if let Some(value) = value {
				println!("- {}={}", key.to_string_lossy(), value.to_string_lossy());
			} else {
				println!("- {}", key.to_string_lossy());
			}
		}

		println!();
	}

	let args = command.get_args();

	if args.len() != 0 {
		println!("Running Cargo with arguments:");

		for arg in args {
			println!("- {}", arg.to_string_lossy());
		}

		println!();
	}
}

pub fn run(mut command: Command, verbose: bool) -> Result<(Duration, ExitStatus)> {
	let start = Instant::now();

	let status = if verbose {
		command.status()?
	} else {
		let output = command.output()?;

		if output.status.success() {
			let style = Style::new().fg_color(Some(AnsiColor::Green.into()));
			println!(" {style}ok{style:#}");
		} else {
			let style = Style::new().fg_color(Some(AnsiColor::Red.into()));
			println!(" {style}failed{style:#}");
			println!();

			if !output.stdout.is_empty() {
				eprintln!(
					"------ Cargo stdout ------\n{}",
					String::from_utf8_lossy(&output.stdout)
				);

				if !output.stdout.ends_with(b"\n") {
					eprintln!();
				}
			}

			if !output.stderr.is_empty() {
				eprintln!(
					"------ Cargo stderr ------\n{}",
					String::from_utf8_lossy(&output.stderr)
				);

				if !output.stderr.ends_with(b"\n") {
					eprintln!();
				}
			}
		}

		output.status
	};

	Ok((start.elapsed(), status))
}

#[derive(Clone)]
pub struct ToolchainParser;

impl TypedValueParser for ToolchainParser {
	type Value = String;

	fn parse_ref(
		&self,
		cmd: &clap::Command,
		arg: Option<&Arg>,
		value: &OsStr,
	) -> Result<Self::Value, Error> {
		TypedValueParser::parse(self, cmd, arg, value.to_owned())
	}

	fn parse(
		&self,
		cmd: &clap::Command,
		arg: Option<&Arg>,
		value: OsString,
	) -> Result<Self::Value, Error> {
		let value = StringValueParser::parse(&StringValueParser::new(), cmd, arg, value)?;

		if value.chars().any(char::is_whitespace) {
			let mut error = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);

			if let Some(arg) = arg {
				error.insert(
					ContextKind::InvalidArg,
					ContextValue::String(arg.to_string()),
				);
			}

			error.insert(
				ContextKind::InvalidValue,
				ContextValue::String(String::from("contains spaces")),
			);

			Err(error)
		} else {
			Ok(value)
		}
	}
}
