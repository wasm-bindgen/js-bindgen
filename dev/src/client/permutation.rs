use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter};
use std::{env, iter, mem};

use strum::{EnumIter, IntoEnumIterator};

use super::{Target, TargetFeature};

pub struct Permutation {
	target: Target,
	target_feature: TargetFeature,
	js_sys_target_feature: Option<JsSysTargetFeature>,
	rustflags: Option<Rustflags>,
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
		js_sys: bool,
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
					.chain(
						js_sys
							.then_some(JsSysTargetFeature::iter().map(Some))
							.into_iter()
							.flatten(),
					)
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

	pub fn target(&self) -> Target {
		self.target
	}

	pub fn target_feature(&self) -> TargetFeature {
		self.target_feature
	}

	pub fn js_sys_target_feature(&self) -> Option<JsSysTargetFeature> {
		self.js_sys_target_feature
	}

	pub fn fmt(
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

#[derive(Clone, Copy, EnumIter)]
pub enum JsSysTargetFeature {
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

	pub fn requires_rab(self) -> bool {
		match self {
			Self::NoSharedMemory
			| Self::SmallEndian
			| Self::BigEndian
			| Self::Sab
			| Self::NoSharedMemorySab => false,
			Self::Rab | Self::SmallEndianRab | Self::BigEndianRab | Self::AllFeatures => true,
		}
	}

	pub fn requires_sab(self) -> bool {
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
