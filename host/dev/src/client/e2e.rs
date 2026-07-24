use std::fmt::Write as _;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use cargo_metadata::{Artifact, Message, TargetKind};
use tempfile::TempDir;

use super::permutation::Permutation;
use super::test::Engine;
use super::util;
use crate::command;

pub struct E2e {
	_dir: TempDir,
	examples: Vec<Example>,
}

struct Example {
	dir: PathBuf,
	name: String,
	tests: Vec<String>,
}

impl E2e {
	pub fn build(
		permutation: &Permutation,
		nightly_toolchain: &str,
		verbose: bool,
	) -> Result<(Self, Duration)> {
		let start = Instant::now();
		let mut command = util::cargo(permutation, nightly_toolchain, "build");
		command
			.args(["-p", "js-bindgen-e2e", "--examples"])
			.arg("--message-format=json-render-diagnostics");
		let output = command.output().context("failed to build E2E examples")?;
		let mut artifacts = Vec::new();

		for message in Message::parse_stream(Cursor::new(output.stdout)) {
			match message? {
				Message::CompilerArtifact(artifact)
					if artifact.target.kind.contains(&TargetKind::Example) =>
				{
					artifacts.push(artifact);
				}
				Message::CompilerMessage(message) if verbose => {
					if let Some(rendered) = message.message.rendered {
						eprint!("{rendered}");
					}
				}
				_ => {}
			}
		}

		if !output.status.success() {
			if !output.stderr.is_empty() {
				eprint!("{}", String::from_utf8_lossy(&output.stderr));
			}

			bail!("building E2E examples failed with {}", output.status);
		}

		ensure!(!artifacts.is_empty(), "no E2E examples were built");
		let dir = tempfile::tempdir().context("failed to create E2E output directory")?;
		let mut examples = Vec::with_capacity(artifacts.len());

		for artifact in artifacts {
			examples.push(Example::build(artifact, dir.path(), verbose)?);
		}

		examples.sort_unstable_by(|left, right| left.name.cmp(&right.name));

		Ok((
			Self {
				_dir: dir,
				examples,
			},
			start.elapsed(),
		))
	}

	pub fn run(
		&self,
		engine: Engine,
		node_js_arg: Option<&str>,
		verbose: bool,
	) -> Result<Duration> {
		let mut duration = Duration::ZERO;

		for example in &self.examples {
			let script = example.script(engine);
			let script_path = example.dir.join("test.mjs");
			fs::write(&script_path, script).with_context(|| {
				format!("failed to write E2E script: {}", script_path.display())
			})?;

			let mut command = Command::new(engine.binary());

			match engine {
				Engine::Deno => {
					command.args(["run", "--allow-read"]);
				}
				Engine::NodeJs => {
					command.args(node_js_arg);
				}
				Engine::Bun => {
					command.arg("run");
				}
			}

			command.arg(script_path);
			duration += command::run(
				&format!("E2E `{}` - {engine}", example.name),
				command,
				verbose,
			)?;
		}

		Ok(duration)
	}
}

impl Example {
	fn build(artifact: Artifact, output: &Path, verbose: bool) -> Result<Self> {
		let wasm = artifact
			.filenames
			.iter()
			.find(|path| path.extension() == Some("wasm"))
			.with_context(|| {
				format!(
					"E2E example `{}` did not produce a Wasm artifact",
					artifact.target.name
				)
			})?;
		let source = fs::read_to_string(&artifact.target.src_path)
			.with_context(|| format!("failed to read E2E example: {}", artifact.target.src_path))?;
		let tests: Vec<_> = source
			.lines()
			.filter_map(|line| line.trim_start().strip_prefix("// ;;"))
			.map(str::trim)
			.map(str::to_owned)
			.collect();

		ensure!(
			!tests.is_empty(),
			"E2E example `{}` has no `// ;;` tests",
			artifact.target.name
		);
		ensure!(
			tests.iter().all(|test| !test.is_empty()),
			"E2E example `{}` contains an empty `// ;;` test",
			artifact.target.name
		);

		let dir = output.join(&artifact.target.name);
		fs::create_dir(&dir)
			.with_context(|| format!("failed to create E2E directory: {}", dir.display()))?;
		let mut command = if std::env::var_os("JBG_DEV_TOOLS").is_some_and(|value| value == "1") {
			Command::new("js-bindgen")
		} else {
			let mut command = Command::new("cargo");
			command
				.current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
				.args(["+stable", "run", "-q", "-p", "js-bindgen-cli", "--"]);
			command
		};
		command.arg(wasm).arg("--out-dir").arg(&dir);
		command::run(
			&format!("Generate E2E `{}`", artifact.target.name),
			command,
			verbose,
		)?;

		Ok(Self {
			dir,
			name: artifact.target.name,
			tests,
		})
	}

	fn script(&self, engine: Engine) -> String {
		let read = match engine {
			Engine::Deno => {
				format!(
					"const bytes = await Deno.readFile(new URL('./{}.wasm', import.meta.url))",
					self.name
				)
			}
			Engine::NodeJs => format!(
				"const {{ readFile }} = await import('node:fs/promises')\nconst bytes = await \
				 readFile(new URL('./{}.wasm', import.meta.url))",
				self.name
			),
			Engine::Bun => format!(
				"const bytes = await Bun.file(new URL('./{}.wasm', import.meta.url)).arrayBuffer()",
				self.name
			),
		};
		let mut script = format!(
			"import {{ JsBindgen }} from './{}.mjs'\n\n{read}\nconst module = await \
			 WebAssembly.compile(bytes)\nconst {{ exports }} = await new \
			 JsBindgen(module).instantiate()\n\nfunction assert(value, expression) {{\n    if \
			 (!value) throw new Error(`assertion failed: ${{expression}}`)\n}}\n",
			self.name
		);

		for test in &self.tests {
			script.push_str("\nassert(");
			script.push_str(test);
			script.push_str(", ");
			write!(script, "{test:?}").unwrap();
			script.push_str(")\n");
		}

		script
	}
}
