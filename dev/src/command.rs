use std::process::{Command, ExitStatus};
use std::time::{Duration, Instant};

use anstyle::{AnsiColor, Style};
use anyhow::Result;

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

	command.env("CARGO_TERM_COLOR", "always");

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

pub struct RunCommand<'a> {
	pub title: &'a str,
	pub sub_command: &'a str,
	pub envs: &'a [(&'a str, &'a str)],
	pub args: &'a [&'a str],
}
