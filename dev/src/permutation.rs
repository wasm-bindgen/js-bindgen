use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter};
use std::{env, iter, mem};

use strum::{EnumIter, IntoEnumIterator};

use crate::{Engine, Runner, Target, TargetFeature, WebDriver};

pub struct Permutation {
	target: Target,
	target_feature: TargetFeature,
	js_sys_target_feature: Option<JsSysTargetFeature>,
	rustflags: Option<Rustflags>,
}

pub struct TestRun {
	target: Target,
	target_feature: TargetFeature,
	js_sys_target_feature: Option<JsSysTargetFeature>,
	runner: Runner,
	remote: Option<WebDriver>,
	worker: Option<Worker>,
	node_js_arg: Option<&'static str>,
}

#[derive(Clone, Default)]
struct Rustflags {
	rust: OsString,
	doc: OsString,
}

impl Permutation {
	pub fn iter(
		targets: &[Target],
		target_features: &[TargetFeature],
	) -> impl Iterator<Item = Self> {
		targets
			.iter()
			.copied()
			.flat_map(|target| iter::repeat(target).zip(target_features.iter().copied()))
			.flat_map(move |(target, target_feature)| {
				let mut orig_rustflags = Rustflags::original(target);
				let mut rustflags = None;

				if let Some(flags) = target_feature.flags() {
					rustflags
						.get_or_insert(mem::take(&mut orig_rustflags))
						.push(flags);
				}

				iter::once(None)
					.chain(JsSysTargetFeature::iter().map(Some))
					.filter(move |js_sys_target_feature| {
						!js_sys_target_feature.is_some_and(JsSysTargetFeature::requires_atomic)
							|| target_feature.supports_atomics()
					})
					.map(move |js_sys_target_feature| {
						let mut rustflags = rustflags.clone();

						if let Some(flags) =
							js_sys_target_feature.and_then(JsSysTargetFeature::flags)
						{
							rustflags
								.get_or_insert_with(|| orig_rustflags.clone())
								.push(flags);
						}

						if target_feature.supports_atomics()
							&& js_sys_target_feature.is_none_or(JsSysTargetFeature::shared_memory)
						{
							rustflags
								.get_or_insert_with(|| orig_rustflags.clone())
								.push("-Clink-arg=--shared-memory");
						}

						Self {
							target,
							target_feature,
							js_sys_target_feature,
							rustflags,
						}
					})
			})
	}

	pub fn test_runs(
		&self,
		filter: impl Copy + Fn(&Runner) -> bool,
	) -> impl Iterator<Item = TestRun> {
		Runner::iter()
			.filter(filter)
			.filter(|runner| runner.supports_target(self.target))
			.filter(|runner| {
				!self
					.js_sys_target_feature
					.is_some_and(JsSysTargetFeature::requires_rab)
					|| runner.supports_rab()
			})
			.filter(|runner| {
				!self
					.js_sys_target_feature
					.is_some_and(JsSysTargetFeature::requires_sab)
					|| runner.supports_sab()
			})
			.flat_map(move |runner| {
				let node_js_arg = (matches!(runner, Runner::Engine(Engine::NodeJs))
					&& self
						.js_sys_target_feature
						.is_some_and(JsSysTargetFeature::requires_rab))
				.then_some("--experimental-wasm-rab-integration");
				let remote = if let Runner::WebDriver(web_driver) = runner {
					Some(web_driver)
				} else {
					None
				};

				iter::once(TestRun {
					target: self.target,
					target_feature: self.target_feature,
					js_sys_target_feature: self.js_sys_target_feature,
					runner,
					remote,
					worker: None,
					node_js_arg,
				})
				.chain(
					matches!(runner, Runner::WebDriver(_))
						.then(|| {
							Worker::iter().map(move |worker| TestRun {
								target: self.target,
								target_feature: self.target_feature,
								js_sys_target_feature: self.js_sys_target_feature,
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

	pub fn envs(&self) -> impl Iterator<Item = (&str, &OsStr)> {
		self.rustflags.iter().flat_map(|rustflags| {
			[
				(self.target.rustflags_env(), rustflags.rust.as_ref()),
				(self.target.rustdocflags_env(), rustflags.doc.as_ref()),
			]
		})
	}

	pub fn toolchain(&self) -> Toolchain {
		self.target.toolchain(self.target_feature)
	}

	pub fn args(&self) -> &'static [&'static str] {
		self.target.args(self.target_feature)
	}

	fn fmt(
		target: Target,
		target_feature: TargetFeature,
		js_sys_target_feature: Option<JsSysTargetFeature>,
		f: &mut Formatter<'_>,
	) -> fmt::Result {
		target.fmt(f)?;

		if !matches!(target_feature, TargetFeature::Default) {
			write!(f, " {target_feature}")?;
		}

		if let Some(js_sys_target_feature) = js_sys_target_feature {
			write!(f, " {js_sys_target_feature}")
		} else {
			Ok(())
		}
	}
}

impl Display for Permutation {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Self::fmt(
			self.target,
			self.target_feature,
			self.js_sys_target_feature,
			f,
		)
	}
}

impl TestRun {
	pub fn envs(&self) -> impl Iterator<Item = (&str, &OsStr)> {
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

#[derive(Clone, Copy, EnumIter)]
enum JsSysTargetFeature {
	NoSharedMemory,
	SmallEndian,
	BigEndian,
	Sab,
	NoSharedMemorySab,
	Rab,
	SmallEndianRab,
	BigEndianRab,
	AllFeatures,
}

impl JsSysTargetFeature {
	fn flags(self) -> Option<&'static str> {
		Some(match self {
			Self::NoSharedMemory => return None,
			Self::SmallEndian => "--cfg js_sys_assume_endianness=\"little\"",
			Self::BigEndian => "--cfg js_sys_assume_endianness=\"big\"",
			Self::Sab | Self::NoSharedMemorySab => "--cfg js_sys_target_feature=\"sab\"",
			Self::Rab => "--cfg js_sys_target_feature=\"unstable-rab\"",
			Self::SmallEndianRab => {
				"--cfg js_sys_assume_endianness=\"little\" --cfg \
				 js_sys_target_feature=\"unstable-rab\""
			}
			Self::BigEndianRab => {
				"--cfg js_sys_assume_endianness=\"big\" --cfg \
				 js_sys_target_feature=\"unstable-rab\""
			}
			Self::AllFeatures => {
				"--cfg js_sys_target_feature=\"sab\" --cfg js_sys_target_feature=\"unstable-rab\""
			}
		})
	}

	fn shared_memory(self) -> bool {
		match self {
			Self::SmallEndian
			| Self::BigEndian
			| Self::Sab
			| Self::Rab
			| Self::SmallEndianRab
			| Self::BigEndianRab
			| Self::AllFeatures => true,
			Self::NoSharedMemory | Self::NoSharedMemorySab => false,
		}
	}

	fn requires_rab(self) -> bool {
		match self {
			Self::NoSharedMemory
			| Self::SmallEndian
			| Self::BigEndian
			| Self::Sab
			| Self::NoSharedMemorySab => false,
			Self::Rab | Self::SmallEndianRab | Self::BigEndianRab | Self::AllFeatures => true,
		}
	}

	fn requires_sab(self) -> bool {
		match self {
			Self::NoSharedMemory
			| Self::SmallEndian
			| Self::BigEndian
			| Self::Rab
			| Self::NoSharedMemorySab
			| Self::SmallEndianRab
			| Self::BigEndianRab => false,
			Self::Sab | Self::AllFeatures => true,
		}
	}

	fn requires_atomic(self) -> bool {
		match self {
			Self::SmallEndian
			| Self::BigEndian
			| Self::Rab
			| Self::SmallEndianRab
			| Self::BigEndianRab => false,
			Self::NoSharedMemory | Self::Sab | Self::NoSharedMemorySab | Self::AllFeatures => true,
		}
	}
}

impl Display for JsSysTargetFeature {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::NoSharedMemory => "No SM",
			Self::SmallEndian => "Small Endian",
			Self::BigEndian => "Big Endian",
			Self::Sab => "SAB",
			Self::NoSharedMemorySab => "SAB + No SM",
			Self::Rab => "RAB",
			Self::SmallEndianRab => "Small Endian + RAB",
			Self::BigEndianRab => "Big Endian + RAB",
			Self::AllFeatures => "All Target Features",
		};

		f.write_str(name)
	}
}

impl Rustflags {
	fn original(target: Target) -> Self {
		Self {
			rust: env::var_os(target.rustflags_env()).unwrap_or_default(),
			doc: env::var_os(target.rustdocflags_env()).unwrap_or_default(),
		}
	}

	fn push(&mut self, str: &str) {
		if !self.rust.is_empty() {
			self.rust.push(" ");
		}

		if !self.doc.is_empty() {
			self.doc.push(" ");
		}

		self.rust.push(str);
		self.doc.push(str);
	}
}

#[derive(Clone, Copy)]
pub enum Toolchain {
	Any,
	Nightly,
}
