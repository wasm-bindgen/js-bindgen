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
				.expect("package manifest should be in a folder")
				.as_std_path();

			for target in package
				.targets
				.iter()
				.filter(|target| !target.is_custom_build())
			{
				let folder = target
					.src_path
					.parent()
					.expect("target source file should be in a folder")
					.as_std_path();

				let mut state = State {
					summary: &mut summary,
					base,
					global_args,
					package: &package.name,
					crate_: &crate_,
					js_sys: js_sys.as_ref(),
				};

				match state.process(folder)? {
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
				"{style}{:>9}:{style:#} Total {}, {} {}, Skipped {}, Errors {}",
				"Summary",
				summary.generated + summary.skipped + summary.errors,
				if global_args.dry_run {
					"Planned"
				} else {
					"Generated"
				},
				summary.generated,
				summary.skipped,
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
	skipped: usize,
	errors: usize,
}

impl Summary {
	fn new() -> Self {
		Self {
			generated: 0,
			skipped: 0,
			errors: 0,
		}
	}
}

impl State<'_> {
	fn process(&mut self, folder: &Path) -> Result<ControlFlow<(), bool>> {
		let mut success = true;

		for entry in fs::read_dir(folder)? {
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
		let output_file = entry.with_extension("").with_extension("rs");
		let relative_output_file = output_file.strip_prefix(self.base).unwrap_or(&output_file);
		let exists = output_file.exists();

		if exists && !output_file.is_file() {
			let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
			eprintln!(
				"{style}{:>9}:{style:#} output file exists and is not a file: {}",
				"Error",
				relative_output_file.display()
			);

			self.summary.errors += 1;

			return Ok(false);
		}

		if !exists || ReadFile::new(&output_file)?.deref() != output.as_bytes() {
			if !self.global_args.dry_run {
				fs::write(&output_file, output)?;
			}

			if !self.global_args.quiet {
				let style = Style::new().fg_color(Some(AnsiColor::Green.into()));
				println!(
					"{style}{:>9}:{style:#} {} -> {}",
					if self.global_args.dry_run {
						"Planned"
					} else {
						"Generated"
					},
					relative_entry.display(),
					relative_output_file.display()
				);
			}

			self.summary.generated += 1;
		} else {
			if self.global_args.verbose {
				let style = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
				println!(
					"{style}{:>9}:{style:#} {} -> {}",
					"Unchanged",
					relative_entry.display(),
					relative_output_file.display()
				);
			}

			self.summary.skipped += 1;
		}

		Ok(true)
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
