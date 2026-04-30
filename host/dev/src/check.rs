use std::iter::{self, Copied};
use std::process::Command;
use std::slice::Iter;
use std::sync::LazyLock;
use std::time::Instant;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Args, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use crate::client::{self, Client, ClientTool};
use crate::command;
use crate::host::{self, Host, HostTool};

#[derive(Args)]
pub struct Check {
	#[arg(long, value_delimiter = ',', default_value = Tools::default_arg())]
	tools: Vec<Tools>,
	#[arg(long)]
	all: bool,
}

enum_with_all!(enum Tools, Tool(Tool), "tools");

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tool {
	Shared(CheckTool),
	Client(ClientTool),
	Host(HostTool),
	Zizmor,
}

#[derive(Clone, Copy, Default, EnumIter, Eq, PartialEq, ValueEnum)]
pub enum CheckTool {
	#[default]
	Clippy,
	RustSec,
	Tombi,
	CargoSpellcheck,
	Typos,
}

enum RootTool {
	Tombi,
	CargoSpellcheck,
	Typos,
	Zizmor,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			tools: vec![Tools::default()],
			all: false,
		}
	}
}

impl Check {
	pub fn all() -> Self {
		Self {
			tools: Tools::all(),
			all: true,
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let tools = Tool::from_tools(self.tools)?;
		let mut client_tools = Vec::new();
		let mut host_tools = Vec::new();
		let mut root_tools = Vec::new();

		for tool in tools {
			match tool {
				Tool::Shared(tool) => match tool {
					CheckTool::Clippy | CheckTool::RustSec => {
						client_tools.push(client::Tool::Shared(tool));
						host_tools.push(host::Tool::Shared(tool));
					}
					CheckTool::Tombi => root_tools.push(RootTool::Tombi),
					CheckTool::CargoSpellcheck => root_tools.push(RootTool::CargoSpellcheck),
					CheckTool::Typos => root_tools.push(RootTool::Typos),
				},
				Tool::Client(tool) => client_tools.push(client::Tool::Client(tool)),
				Tool::Host(tool) => host_tools.push(host::Tool::Host(tool)),
				Tool::Zizmor => root_tools.push(RootTool::Zizmor),
			}
		}

		Client::check(client_tools, self.all).execute(verbose)?;
		println!("-------------------------");
		println!();
		Host::check(host_tools, self.all).execute(verbose)?;
		println!("-------------------------");
		println!();

		let start = Instant::now();

		for tool in root_tools {
			match tool {
				RootTool::Tombi => {
					let mut command = Command::new("tombi");
					command
						.current_dir("..")
						.args(["lint", "--error-on-warnings", "."]);
					command::run("Tombi Lint", command, verbose)?;
				}
				RootTool::CargoSpellcheck => {
					let mut command = Command::new("cargo");
					command.current_dir("..").args(["spellcheck", "-m", "1"]);
					command::run("`cargo-spellcheck`", command, verbose)?;
				}
				RootTool::Typos => {
					let mut command = Command::new("typos");
					command.current_dir("..");
					command::run("Typos", command, verbose)?;
				}
				RootTool::Zizmor => {
					let mut command = Command::new("zizmor");
					command.current_dir("../").args(["--pedantic", "."]);
					command::run("Zizmor", command, verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

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
				.chain(ClientTool::iter().map(Tool::Client))
				.chain(HostTool::iter().map(Tool::Host))
				.chain(iter::once(Tool::Zizmor))
				.collect()
		});

		&VALUES
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::Shared(tool) => tool.to_possible_value(),
			Self::Client(tool) => tool.to_possible_value(),
			Self::Host(tool) => tool.to_possible_value(),
			Self::Zizmor => Some(PossibleValue::new("zizmor")),
		}
	}
}
