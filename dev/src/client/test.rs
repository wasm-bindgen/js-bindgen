use std::ffi::OsStr;
use std::fmt::{self, Display, Formatter};
use std::process::{Command, Stdio};
use std::sync::LazyLock;
use std::time::{Duration, Instant};
use std::{env, iter};

use anyhow::{Result, anyhow, bail};
use clap::builder::{ArgPredicate, PossibleValue};
use clap::{Args, ValueEnum};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use super::permutation::{JsSysTargetFeature, Permutation};
use super::process::ChildWrapper;
use super::{ClientArgs, Target, TargetFeature, util};
use crate::command;
use crate::group::Group;

#[derive(Args)]
pub struct Test {
	#[command(flatten)]
	args: ClientArgs,
	#[arg(
		long,
		value_delimiter = ',',
		conflicts_with = "exclude",
		default_value = "engine",
		default_values_if("exclude", ArgPredicate::IsPresent, iter::empty::<&str>()),
		required = false
	)]
	include: Vec<Include>,
	#[arg(long, value_delimiter = ',', conflicts_with = "include")]
	exclude: Vec<Runner>,
}

impl Default for Test {
	fn default() -> Self {
		Self {
			args: ClientArgs::default(),
			include: vec![Include::Engine(None)],
			exclude: Vec::new(),
		}
	}
}

impl Test {
	pub fn all() -> Self {
		Self {
			args: ClientArgs::all(),
			include: vec![Include::All],
			exclude: Vec::new(),
		}
	}

	pub fn args(&self) -> &ClientArgs {
		&self.args
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let runners = if !self.include.is_empty() {
			Runner::from_include(self.include)?
		} else if !self.exclude.is_empty() {
			Runner::iter()
				.filter(|runner| !self.exclude.contains(runner))
				.collect()
		} else {
			unreachable!()
		};

		let tools_installed = env::var_os("JBG_DEV_TOOLS").is_some_and(|value| value == "1");

		let start = Instant::now();
		let mut build_time = Duration::ZERO;
		let mut test_time = Duration::ZERO;

		let mut web_drivers = Vec::new();
		let mut session_manager_built = tools_installed;

		for web_driver in runners.iter().filter_map(|runner| {
			if let Runner::WebDriver(web_driver) = runner {
				Some(web_driver)
			} else {
				None
			}
		}) {
			if !session_manager_built {
				let group = Group::announce("Build Session Manager".into(), verbose)?;
				let mut command = Command::new("cargo");
				command
					.current_dir("../host")
					.arg("build")
					.args(["-p", "js-bindgen-web-driver"]);

				let (duration, status) = command::run(command, verbose)?;
				build_time += duration;

				if !status.success() {
					bail!("build Session Manager failed with {status}");
				}

				drop(group);
				session_manager_built = true;
			}

			let mut command = if tools_installed {
				Command::new("js-bindgen-web-driver")
			} else {
				let mut command = Command::new("cargo");
				command
					.current_dir("../host")
					.arg("run")
					.args(["-p", "js-bindgen-web-driver"])
					.arg("--");
				command
			};

			command
				.stdout(Stdio::null())
				.stderr(Stdio::null())
				.args(["-p", web_driver.port()])
				.arg(web_driver.short_name());

			web_drivers.push(ChildWrapper::new(command)?);
		}

		if !tools_installed {
			build_time += util::build_linker(verbose)?.unwrap();

			let group = Group::announce("Build Runner".into(), verbose)?;
			let mut command = Command::new("cargo");
			command
				.current_dir("../host")
				.arg("build")
				.args(["-p", "js-bindgen-runner"]);

			let (duration, status) = command::run(command, verbose)?;
			build_time += duration;

			if !status.success() {
				bail!("build Runner failed with {status}");
			}

			drop(group);
		}

		for permutation in Permutation::iter(&self.args.targets, &self.args.target_features, true) {
			let mut built = false;

			for test_run in TestRun::from_permuation(&permutation, &runners) {
				if !built {
					let group =
						Group::announce(format!("Build Tests - {permutation}").into(), verbose)?;
					let mut command =
						util::cargo(&permutation, &self.args.nightly_toolchain, "test");
					command
						.arg("--workspace")
						.arg("--all-features")
						.arg("--no-run");

					if verbose {
						command::print_info(&command);
					}

					let (duration, status) = command::run(command, verbose)?;
					build_time += duration;

					if !status.success() {
						bail!("build \"{permutation}\" failed with {status}");
					}

					drop(group);
					built = true;
				}

				let group = Group::announce(format!("Run Tests - {test_run}").into(), verbose)?;
				let mut command = util::cargo(&permutation, &self.args.nightly_toolchain, "test");
				command
					.envs(test_run.envs())
					.arg("--workspace")
					.arg("--all-features");

				if verbose {
					command::print_info(&command);
				}

				let (duration, status) = command::run(command, verbose)?;
				test_time += duration;

				if !status.success() {
					bail!("test \"{test_run}\" failed with {status}");
				}

				drop(group);
			}
		}

		for web_driver in web_drivers {
			web_driver.shutdown()?;
		}

		println!("-------------------------");
		println!("Build Time: {:.2}s", build_time.as_secs_f32());
		println!("Test Time: {:.2}s", test_time.as_secs_f32());
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Include {
	All,
	Engine(Option<Engine>),
	WebDriver(WebDriver),
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Runner {
	Engine(Engine),
	WebDriver(WebDriver),
}

impl Runner {
	fn from_include(cli: Vec<Include>) -> Result<Vec<Self>> {
		if let [Include::All] = cli.as_slice() {
			return Ok(Self::iter().collect());
		}

		cli.into_iter()
			.map(|runner| match runner {
				Include::All => Err(anyhow!(
					"`--include`s `all` option conflicts with all others"
				)),
				Include::Engine(None) => {
					for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
						for engine in Engine::iter() {
							if path
								.join(engine.binary())
								.with_extension(env::consts::EXE_EXTENSION)
								.exists()
							{
								return Ok(Self::Engine(engine));
							}
						}
					}

					Err(anyhow!("failed to find a suitable JS engine binary"))
				}
				Include::Engine(Some(engine)) => Ok(Self::Engine(engine)),
				Include::WebDriver(web_driver) => Ok(Self::WebDriver(web_driver)),
			})
			.collect()
	}

	fn iter() -> impl Iterator<Item = Self> {
		Engine::iter()
			.map(Self::Engine)
			.chain(WebDriver::iter().map(Self::WebDriver))
	}

	fn env(self) -> &'static str {
		match self {
			Self::Engine(engine) => engine.env(),
			Self::WebDriver(web_driver) => web_driver.runner_env(),
		}
	}

	fn supports_target(self, target: Target) -> bool {
		match target {
			Target::Wasm32 => true,
			Target::Wasm64 => match self {
				Self::Engine(_) | Self::WebDriver(WebDriver::Chrome | WebDriver::Gecko) => true,
				#[cfg(target_os = "macos")]
				Self::WebDriver(WebDriver::Safari) => false,
			},
		}
	}

	fn supports_rab(self) -> bool {
		match self {
			Self::Engine(_) => true,
			Self::WebDriver(WebDriver::Chrome | WebDriver::Gecko) => false,
			#[cfg(target_os = "macos")]
			Self::WebDriver(WebDriver::Safari) => false,
		}
	}

	fn supports_sab(self) -> bool {
		match self {
			Self::Engine(_) => true,
			Self::WebDriver(WebDriver::Chrome | WebDriver::Gecko) => false,
			#[cfg(target_os = "macos")]
			Self::WebDriver(WebDriver::Safari) => true,
		}
	}
}

impl Display for Runner {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Engine(engine) => engine.fmt(f),
			Self::WebDriver(web_driver) => web_driver.fmt(f),
		}
	}
}

impl ValueEnum for Include {
	fn value_variants<'a>() -> &'a [Self] {
		static VARIANTS: LazyLock<Vec<Include>> = LazyLock::new(|| {
			[Include::All, Include::Engine(None)]
				.into_iter()
				.chain(Engine::iter().map(Some).map(Include::Engine))
				.chain(WebDriver::iter().map(Include::WebDriver))
				.collect()
		});

		&VARIANTS
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::All => Some(PossibleValue::new("all")),
			Self::Engine(None) => Some(PossibleValue::new("engine")),
			Self::Engine(Some(engine)) => Some(PossibleValue::new(engine.env())),
			Self::WebDriver(web_driver) => Some(PossibleValue::new(web_driver.runner_env())),
		}
	}
}

impl ValueEnum for Runner {
	fn value_variants<'a>() -> &'a [Self] {
		static VARIANTS: LazyLock<Vec<Runner>> = LazyLock::new(|| {
			Engine::iter()
				.map(Runner::Engine)
				.chain(WebDriver::iter().map(Runner::WebDriver))
				.collect()
		});

		&VARIANTS
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		Some(PossibleValue::new(self.env()))
	}
}

#[derive(Clone, Copy, EnumCount, EnumIter, Eq, PartialEq)]
enum Engine {
	Deno,
	NodeJs,
}

impl Engine {
	fn env(self) -> &'static str {
		match self {
			Self::Deno => "deno",
			Self::NodeJs => "node-js",
		}
	}

	fn binary(self) -> &'static str {
		match self {
			Self::Deno => "deno",
			Self::NodeJs => "node",
		}
	}
}

impl Display for Engine {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Deno => "Deno",
			Self::NodeJs => "Node.js",
		};

		f.write_str(name)
	}
}

#[derive(Clone, Copy, EnumCount, EnumIter, Eq, PartialEq)]
enum WebDriver {
	Chrome,
	Gecko,
	#[cfg(target_os = "macos")]
	Safari,
}

impl WebDriver {
	fn runner_env(self) -> &'static str {
		match self {
			Self::Chrome => "chrome-driver",
			Self::Gecko => "gecko-driver",
			#[cfg(target_os = "macos")]
			Self::Safari => "safari-driver",
		}
	}

	fn remote_env(self) -> &'static str {
		match self {
			Self::Chrome => "JBG_TEST_CHROME_DRIVER_REMOTE",
			Self::Gecko => "JBG_TEST_GECKO_DRIVER_REMOTE",
			#[cfg(target_os = "macos")]
			Self::Safari => "JBG_TEST_SAFARI_DRIVER_REMOTE",
		}
	}

	fn remote_url(self) -> &'static str {
		match self {
			Self::Chrome => "http://127.0.0.1:8000",
			Self::Gecko => "http://127.0.0.1:8001",
			#[cfg(target_os = "macos")]
			Self::Safari => "http://127.0.0.1:8002",
		}
	}

	fn short_name(self) -> &'static str {
		match self {
			Self::Chrome => "chrome",
			Self::Gecko => "gecko",
			#[cfg(target_os = "macos")]
			Self::Safari => "safari",
		}
	}

	fn port(self) -> &'static str {
		match self {
			Self::Chrome => "8000",
			Self::Gecko => "8001",
			#[cfg(target_os = "macos")]
			Self::Safari => "8002",
		}
	}
}

impl Display for WebDriver {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Chrome => "Chrome",
			Self::Gecko => "Firefox",
			#[cfg(target_os = "macos")]
			Self::Safari => "Safari",
		};

		f.write_str(name)
	}
}

struct TestRun {
	target: Target,
	target_feature: TargetFeature,
	js_sys_target_feature: Option<JsSysTargetFeature>,
	runner: Runner,
	remote: Option<WebDriver>,
	worker: Option<Worker>,
	node_js_arg: Option<&'static str>,
}

impl TestRun {
	fn from_permuation(
		permutation: &Permutation,
		runners: &[Runner],
	) -> impl Iterator<Item = Self> {
		runners
			.iter()
			.copied()
			.filter(|runner| runner.supports_target(permutation.target()))
			.filter(|runner| {
				!permutation
					.js_sys_target_feature()
					.is_some_and(JsSysTargetFeature::requires_rab)
					|| runner.supports_rab()
			})
			.filter(|runner| {
				!permutation
					.js_sys_target_feature()
					.is_some_and(JsSysTargetFeature::requires_sab)
					|| runner.supports_sab()
			})
			.flat_map(move |runner| {
				let node_js_arg = (matches!(runner, Runner::Engine(Engine::NodeJs))
					&& permutation
						.js_sys_target_feature()
						.is_some_and(JsSysTargetFeature::requires_rab))
				.then_some("--experimental-wasm-rab-integration");
				let remote = if let Runner::WebDriver(web_driver) = runner {
					Some(web_driver)
				} else {
					None
				};

				iter::once(Self {
					target: permutation.target(),
					target_feature: permutation.target_feature(),
					js_sys_target_feature: permutation.js_sys_target_feature(),
					runner,
					remote,
					worker: None,
					node_js_arg,
				})
				.chain(
					matches!(runner, Runner::WebDriver(_))
						.then(|| {
							Worker::iter().map(move |worker| Self {
								target: permutation.target(),
								target_feature: permutation.target_feature(),
								js_sys_target_feature: permutation.js_sys_target_feature(),
								runner,
								remote,
								worker: Some(worker),
								node_js_arg,
							})
						})
						.into_iter()
						.flatten(),
				)
			})
	}

	fn envs(&self) -> impl Iterator<Item = (&str, &OsStr)> {
		[("JBG_TEST_RUNNER", OsStr::new(self.runner.env()))]
			.into_iter()
			.chain(
				self.remote
					.map(|web_driver| (web_driver.remote_env(), web_driver.remote_url().as_ref())),
			)
			.chain(
				self.worker
					.map(|worker| ("JBG_TEST_WORKER", worker.env().as_ref())),
			)
			.chain(
				self.node_js_arg
					.map(|value| ("JBG_TEST_NODE_JS_ARGS", value.as_ref())),
			)
	}
}

impl Display for TestRun {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Permutation::fmt(
			self.target,
			self.target_feature,
			self.js_sys_target_feature,
			f,
		)?;

		write!(f, " - {}", self.runner)?;

		if let Some(worker) = self.worker {
			write!(f, " {worker}")?;
		}

		Ok(())
	}
}

#[derive(Clone, Copy, EnumIter)]
enum Worker {
	Dedicated,
	Shared,
	Service,
}

impl Worker {
	fn env(self) -> &'static str {
		match self {
			Self::Dedicated => "dedicated",
			Self::Shared => "shared",
			Self::Service => "service",
		}
	}
}

impl Display for Worker {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Dedicated => "Dedicated",
			Self::Shared => "Shared",
			Self::Service => "Service",
		};

		write!(f, "{name} Worker")
	}
}
