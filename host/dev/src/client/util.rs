use std::env;
use std::ffi::{OsStr, OsString};
use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use clap::builder::{StringValueParser, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Error};

use super::permutation::{Permutation, Toolchain};
use crate::command;

pub fn cargo(permutation: &Permutation, nightly_toolchain: &str, subcommand: &str) -> Command {
	let mut command = Command::new("cargo");
	command.current_dir("../client").envs(permutation.envs());

	if let Toolchain::Nightly = permutation.toolchain() {
		command.arg(format!("+{nightly_toolchain}"));
	}

	command.arg(subcommand).args(permutation.args());

	command
}

pub fn build_linker(verbose: bool) -> Result<Duration> {
	if env::var_os("JBG_DEV_TOOLS").is_none_or(|value| value != "1") {
		let mut command = Command::new("cargo");
		command.arg("build").args(["-p", "js-bindgen-ld"]);

		command::run("Build Linker", command, verbose)
	} else {
		Ok(Duration::ZERO)
	}
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
