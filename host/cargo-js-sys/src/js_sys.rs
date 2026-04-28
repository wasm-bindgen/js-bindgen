use std::ops::{ControlFlow, Deref};
use std::path::Path;
use std::str::FromStr;
use std::{fs, process, str};

use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use anstyle::{AnsiColor, Style};
use anyhow::Result;
use clap::Args;
use clap_cargo::{Manifest, Workspace};
use js_bindgen_shared::ReadFile;
use js_sys_bindgen::syn::{self, Error, parse_quote};
use similar_asserts::SimpleDiff;

use crate::GlobalArgs;

#[derive(Args)]
pub(crate) struct JsSys {
	#[command(flatten)]
	manifest: Manifest,
	#[command(flatten)]
	workspace: Workspace,
	path: Option<PathWrapper>,
}

#[derive(Clone)]
struct PathWrapper(String);

impl JsSys {
	pub(crate) fn run(self, global_args: GlobalArgs) -> Result<()> {
		let mut summary = Summary::new();
		let mut success = true;

		let metadata = self.manifest.metadata();
		let metadata = metadata.exec()?;
		let (packages, _) = self.workspace.partition_packages(&metadata);

		for package in packages {
			let js_sys: Option<syn::Path> =
				if let Some(path) = self.path.as_ref().map(PathWrapper::path) {
					Some(path)
				} else if package.name == "js-sys" {
					Some(parse_quote!(crate))
				} else if let Some(package) = package
					.dependencies
					.iter()
					.find(|dependency| dependency.name == "js-sys")
				{
					Some(
						syn::parse_str(
							&package
								.rename
								.as_ref()
								.unwrap_or(&package.name)
								.replace('-', "_"),
						)
						.unwrap(),
					)
				} else if let Some(package) = package
					.dependencies
					.iter()
					.find(|dependency| dependency.name == "web-sys")
				{
					let web_sys = package
						.rename
						.as_ref()
						.unwrap_or(&package.name)
						.replace('-', "_");
					Some(syn::parse_str(&format!("{web_sys}::js_sys")).unwrap())
				} else {
					None
				};

			let crate_ = package.name.replace('-', "_");

			let base = package
				.manifest_path
				.parent()
				.expect("package manifest should be in a directory")
				.as_std_path();

			for target in package
				.targets
				.iter()
				.filter(|target| !target.is_custom_build())
			{
				let dir = target
					.src_path
					.parent()
					.expect("target source file should be in a directory")
					.as_std_path();

				let mut state = State {
					summary: &mut summary,
					base,
					global_args,
					package: &package.name,
					crate_: &crate_,
					js_sys: js_sys.as_ref(),
				};

				match state.process(dir)? {
					ControlFlow::Continue(value) => success &= value,
					ControlFlow::Break(()) => {
						success = false;
						break;
					}
				}
			}
		}

		if !global_args.quiet {
			println!();

			let style = Style::new().bold();
			println!(
				"{style}{:>9}:{style:#} Total {}, {} {}, Unchanged {}, Errors {}",
				"Summary",
				summary.generated + summary.unchanged + summary.errors,
				if global_args.check {
					"Checked"
				} else if global_args.dry_run {
					"Planned"
				} else {
					"Generated"
				},
				summary.generated,
				summary.unchanged,
				summary.errors
			);
		}

		if !success {
			process::exit(1);
		}

		Ok(())
	}
}

struct State<'a> {
	summary: &'a mut Summary,
	base: &'a Path,
	global_args: GlobalArgs,
	package: &'a str,
	crate_: &'a str,
	js_sys: Option<&'a syn::Path>,
}

struct Summary {
	generated: usize,
	unchanged: usize,
	errors: usize,
}

impl Summary {
	fn new() -> Self {
		Self {
			generated: 0,
			unchanged: 0,
			errors: 0,
		}
	}
}

impl State<'_> {
	fn process(&mut self, dir: &Path) -> Result<ControlFlow<(), bool>> {
		let mut success = true;

		for entry in fs::read_dir(dir)? {
			let entry = entry?.path();
			let relative_entry = entry.strip_prefix(self.base).unwrap_or(&entry);

			if entry.is_file()
				&& let Some(file) = entry.file_name()
				&& file.as_encoded_bytes().ends_with(b".js-sys.rs")
			{
				let Some(js_sys) = self.js_sys else {
					let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
					eprintln!(
						"{style}Error:{style:#} can't find `js-sys` in dependencies for `{}`, \
						 provide it manually via `--path`",
						self.package
					);
					return Ok(ControlFlow::Break(()));
				};
				let Some(output) = self.generate(js_sys, &entry, relative_entry)? else {
					success = false;
					continue;
				};

				success &= self.output(&entry, relative_entry, &output)?;
			} else if entry.is_dir() {
				match self.process(&entry)? {
					ControlFlow::Continue(value) => success &= value,
					ControlFlow::Break(()) => return Ok(ControlFlow::Break(())),
				}
			}
		}

		Ok(ControlFlow::Continue(success))
	}

	fn generate(
		&mut self,
		js_sys: &syn::Path,
		entry: &Path,
		relative_entry: &Path,
	) -> Result<Option<String>> {
		let input = ReadFile::new(entry)?;
		let input = str::from_utf8(&input)?;
		let output = match js_sys_bindgen::file(input, self.crate_, Some(js_sys.clone())) {
			Ok(output) => output,
			Err(error) => {
				let path = relative_entry.to_string_lossy();
				let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));

				let errors: Vec<_> = error
					.into_iter()
					.map(|error| {
						Level::ERROR
							.no_name()
							.secondary_title(format!("{style}{:>9}:{style:#} {error}", "Error"))
							.element(
								Snippet::source(input)
									.line_start(error.span().start().line)
									.path(&path)
									.annotation(
										AnnotationKind::Primary.span(error.span().byte_range()),
									),
							)
					})
					.collect();

				let output = Renderer::styled()
					.decor_style(DecorStyle::Unicode)
					.render(&errors);
				eprintln!("{output}");

				self.summary.errors += 1;

				return Ok(None);
			}
		};

		Ok(Some(prettyplease::unparse(&output)))
	}

	fn output(&mut self, entry: &Path, relative_entry: &Path, output: &str) -> Result<bool> {
		let output_file = entry.with_extension("").with_extension("gen.rs");
		let relative_output_file = output_file.strip_prefix(self.base).unwrap_or(&output_file);
		let exists = output_file.exists();

		if exists && !output_file.is_file() {
			let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
			eprintln!(
				"{style}{:>9}:{style:#} output file exists but is not a file: {}",
				"Error",
				relative_output_file.display()
			);

			self.summary.errors += 1;

			return Ok(false);
		}

		let current = exists.then(|| ReadFile::new(&output_file)).transpose()?;
		let current = if let Some(current) = &current {
			match str::from_utf8(current.deref()) {
				Ok(current) => Some(current),
				Err(error) => {
					let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
					eprintln!(
						"{style}{:>9}:{style:#} output file exists but is not UTF-8: {}\n\t{error}",
						"Error",
						relative_output_file.display(),
					);

					self.summary.errors += 1;

					return Ok(false);
				}
			}
		} else {
			None
		};

		let feedback = |color: AnsiColor, text: &str| {
			let style = Style::new().fg_color(Some(color.into()));
			println!(
				"{style}{:>9}:{style:#} {} -> {}",
				text,
				relative_entry.display(),
				relative_output_file.display()
			);
		};

		if self.global_args.check {
			let Some(current) = current else {
				feedback(AnsiColor::Red, "Missing");
				self.summary.errors += 1;
				return Ok(false);
			};

			if current == output {
				if self.global_args.verbose {
					feedback(AnsiColor::Green, "Checked");
				}

				self.summary.generated += 1;
				Ok(true)
			} else {
				feedback(AnsiColor::Red, "Different");
				eprintln!(
					"{}",
					SimpleDiff::from_str(current, output, "current", "expected")
				);

				self.summary.errors += 1;
				Ok(false)
			}
		} else if current.is_none_or(|current| current != output) {
			if !self.global_args.dry_run {
				fs::write(&output_file, output)?;
			}

			if !self.global_args.quiet || self.global_args.check {
				feedback(
					AnsiColor::Green,
					if self.global_args.dry_run {
						"Planned"
					} else {
						"Generated"
					},
				);
			}

			self.summary.generated += 1;
			Ok(true)
		} else {
			if self.global_args.verbose {
				feedback(AnsiColor::BrightBlack, "Unchanged");
			}

			self.summary.unchanged += 1;
			Ok(true)
		}
	}
}

impl FromStr for PathWrapper {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		syn::parse_str::<syn::Path>(s)?;
		Ok(Self(s.to_owned()))
	}
}

impl PathWrapper {
	fn path(&self) -> syn::Path {
		syn::parse_str::<syn::Path>(&self.0).unwrap()
	}
}
