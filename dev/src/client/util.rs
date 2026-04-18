use std::ffi::{OsStr, OsString};
use std::process::Command;

use anyhow::Result;
use clap::builder::{StringValueParser, TypedValueParser};
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Arg, Error};

use super::permutation::{Permutation, Toolchain};

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
