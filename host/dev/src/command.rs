use std::io::Write;
use std::process::Command;
use std::time::{Duration, Instant};
use std::{env, io};

use anstyle::{AnsiColor, Style};
use anyhow::{Result, bail};

pub struct CargoCommand<'a> {
	pub title: &'a str,
	pub sub_command: &'a str,
	pub envs: &'a [(&'a str, &'a str)],
	pub args: &'a [&'a str],
}

pub fn run(title: &str, mut command: Command, verbose: bool) -> Result<Duration> {
	let gh_actions = env::var_os("GITHUB_ACTIONS").is_some_and(|value| value == "true");

	if verbose {
		if gh_actions {
			println!("::group::{title}");
		} else {
			println!();
			println!("-------------------------");
			println!("{title}");
			println!("-------------------------");
			println!();
		}
	} else {
		print!("{title} ...");
		io::stdout().flush()?;
	}

	if verbose {
		print_info(&command);
	}

	let start = Instant::now();

	command
		.env("CARGO_TERM_COLOR", "always")
		.env("CLICOLOR_FORCE", "");

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
					"------ `{}` stdout ------\n{}",
					command.get_program().display(),
					String::from_utf8_lossy(&output.stdout)
				);

				if !output.stdout.ends_with(b"\n") {
					eprintln!();
				}
			}

			if !output.stderr.is_empty() {
				eprintln!(
					"------ `{}` stderr ------\n{}",
					command.get_program().display(),
					String::from_utf8_lossy(&output.stderr)
				);

				if !output.stderr.ends_with(b"\n") {
					eprintln!();
				}
			}
		}

		output.status
	};

	let duration = start.elapsed();

	if !status.success() {
		bail!("{title} failed with {status}");
	}

	if gh_actions {
		println!("-------------------------");
		println!("Finished {title}: {:.2}s", duration.as_secs_f32());
		println!("::endgroup::");
	}

	Ok(duration)
}

fn print_info(command: &Command) {
	let envs = command.get_envs();

	if envs.len() != 0 {
		println!(
			"Running `{}` with environment variables:",
			command.get_program().display()
		);

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
		println!(
			"Running `{}` with arguments:",
			command.get_program().display()
		);

		for arg in args {
			println!("- {}", arg.to_string_lossy());
		}

		println!();
	}
}
