use std::borrow::Cow;
use std::io::Write;
use std::time::Instant;
use std::{env, io};

use anyhow::Result;

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
