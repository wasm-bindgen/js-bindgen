use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display, Formatter};
use std::process::Command;

use cargo_metadata::Package;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Features<'f> {
	None,
	Default,
	Some(Vec<&'f str>),
	All,
}

impl Features<'_> {
	pub fn args(&self, command: &mut Command) {
		match self {
			Features::Default => (),
			Features::None => {
				command.arg("--no-default-features");
			}
			Features::Some(features) => {
				command.arg("--no-default-features");

				for feature in features {
					command.args(["-F", feature]);
				}
			}
			Features::All => {
				command.arg("--all-features");
			}
		}
	}
}

impl Display for Features<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Features::Default => Ok(()),
			Features::None => f.write_str("No Default Features"),
			Features::Some(features) => match features.as_slice() {
				[feature] => write!(f, "`feature = {feature}`"),
				features => write!(f, "`features = {}`", features.join(",")),
			},
			Features::All => f.write_str("All Features"),
		}
	}
}

pub fn combinations(package: &Package) -> Vec<Features<'_>> {
	let require_feature = package.metadata.get("dev").is_some_and(|dev| {
		dev.get("require-feature")
			.is_some_and(|value| value.as_bool().is_some_and(|value| value))
	});

	let mut combinations = Vec::new();

	if !require_feature || package.features.contains_key("default") {
		combinations.push(Features::Default);
	}

	if package.features.contains_key("default") && !require_feature {
		combinations.push(Features::None);
	}

	for length in 1..=package.features.len() {
		add(&package.features, length, 0, &mut combinations, &[]);
	}

	combinations
}

fn add<'f>(
	features: &'f BTreeMap<String, Vec<String>>,
	length: usize,
	skip: usize,
	combinations: &mut Vec<Features<'f>>,
	combination: &[&'f str],
) {
	'outer: for (skip, feature) in features.keys().enumerate().skip(skip) {
		if feature == "default" {
			continue;
		}

		if combination.contains(&feature.as_str()) {
			continue;
		}

		let mut combination = combination.to_vec();
		combination.push(feature);

		if length > 1 {
			add(features, length - 1, skip, combinations, &combination);
		} else {
			let expanded = expand(features, combination.iter().copied());
			let all_expanded = expand(features, features.keys().map(String::as_ref));

			for other_combination in combinations.iter() {
				let other_expanded = match other_combination {
					Features::Default => {
						if let Some(combination) = features.get("default") {
							&expand(features, combination.iter().map(String::as_ref))
						} else {
							continue;
						}
					}
					Features::None => continue,
					Features::Some(other_combination) => {
						&expand(features, other_combination.iter().copied())
					}
					Features::All => &all_expanded,
				};

				if &expanded == other_expanded {
					continue 'outer;
				}
			}

			if expanded == all_expanded {
				combinations.push(Features::All);
			} else if let Some(all_position) = combinations
				.iter()
				.position(|feature| matches!(feature, Features::All))
			{
				combinations.insert(all_position, Features::Some(combination));
			} else {
				combinations.push(Features::Some(combination));
			}
		}
	}
}

fn expand<'f>(
	features: &'f BTreeMap<String, Vec<String>>,
	combination: impl Iterator<Item = &'f str>,
) -> BTreeSet<&'f str> {
	fn inner<'f>(
		feature: &'f str,
		features: &'f BTreeMap<String, Vec<String>>,
		expanded: &mut BTreeSet<&'f str>,
	) {
		if feature != "default" {
			expanded.insert(feature);
		}

		for feature in features.get(feature).unwrap() {
			if features.contains_key(feature) {
				inner(feature, features, expanded);
			}
		}
	}

	let mut expanded = BTreeSet::new();

	for feature in combination {
		inner(feature, features, &mut expanded);
	}

	expanded
}

#[cfg(test)]
mod test {
	use cargo_metadata::camino::Utf8PathBuf;
	use cargo_metadata::semver::Version;
	use cargo_metadata::{PackageBuilder, PackageId, PackageName};

	use super::*;

	fn package(features: BTreeMap<String, Vec<String>>) -> Package {
		PackageBuilder::new(
			PackageName::new(String::from("test")),
			Version::parse("0.0.0").unwrap(),
			PackageId {
				repr: String::new(),
			},
			Utf8PathBuf::new(),
		)
		.features(features)
		.build()
		.unwrap()
	}

	#[test]
	fn basic() {
		let mut features = BTreeMap::new();
		features.insert(String::from("a"), Vec::new());
		features.insert(String::from("b"), Vec::new());
		features.insert(String::from("c"), Vec::new());
		features.insert(String::from("d"), Vec::new());
		let package = package(features);

		let combinations = combinations(&package);
		let expected = vec![
			Features::Default,
			Features::Some(vec!["a"]),
			Features::Some(vec!["b"]),
			Features::Some(vec!["c"]),
			Features::Some(vec!["d"]),
			Features::Some(vec!["a", "b"]),
			Features::Some(vec!["a", "c"]),
			Features::Some(vec!["a", "d"]),
			Features::Some(vec!["b", "c"]),
			Features::Some(vec!["b", "d"]),
			Features::Some(vec!["c", "d"]),
			Features::Some(vec!["a", "b", "c"]),
			Features::Some(vec!["a", "b", "d"]),
			Features::Some(vec!["a", "c", "d"]),
			Features::Some(vec!["b", "c", "d"]),
			Features::All,
		];

		assert_eq!(combinations, expected);
	}

	#[test]
	fn none() {
		let package = package(BTreeMap::new());

		let combinations = combinations(&package);
		let expected = vec![Features::Default];

		assert_eq!(combinations, expected);
	}

	#[test]
	fn single() {
		let mut features = BTreeMap::new();
		features.insert(String::from("a"), Vec::new());
		let package = package(features);

		let combinations = combinations(&package);
		let expected = vec![Features::Default, Features::All];

		assert_eq!(combinations, expected);
	}

	#[test]
	fn default() {
		let mut features = BTreeMap::new();
		features.insert(String::from("default"), vec![String::from("a")]);
		features.insert(String::from("a"), Vec::new());
		features.insert(String::from("b"), Vec::new());
		let package = package(features);

		let combinations = combinations(&package);
		let expected = vec![
			Features::Default,
			Features::None,
			Features::Some(vec!["b"]),
			Features::All,
		];

		assert_eq!(combinations, expected);
	}

	#[test]
	fn default_single() {
		let mut features = BTreeMap::new();
		features.insert(String::from("default"), vec![String::from("a")]);
		features.insert(String::from("a"), Vec::new());
		let package = package(features);

		let combinations = combinations(&package);
		let expected = vec![Features::Default, Features::None];

		assert_eq!(combinations, expected);
	}

	#[test]
	fn default_all() {
		let mut features = BTreeMap::new();
		features.insert(
			String::from("default"),
			vec![String::from("a"), String::from("b")],
		);
		features.insert(String::from("a"), Vec::new());
		features.insert(String::from("b"), Vec::new());
		let package = package(features);

		let combinations = combinations(&package);
		let expected = vec![
			Features::Default,
			Features::None,
			Features::Some(vec!["a"]),
			Features::Some(vec!["b"]),
		];

		assert_eq!(combinations, expected);
	}

	#[test]
	fn overlap() {
		let mut features = BTreeMap::new();
		features.insert(String::from("a"), vec![String::from("b")]);
		features.insert(String::from("b"), Vec::new());
		features.insert(String::from("c"), Vec::new());
		features.insert(String::from("d"), Vec::new());
		let package = package(features);

		let combinations = combinations(&package);
		let expected = vec![
			Features::Default,
			Features::Some(vec!["a"]),
			Features::Some(vec!["b"]),
			Features::Some(vec!["c"]),
			Features::Some(vec!["d"]),
			Features::Some(vec!["a", "c"]),
			Features::Some(vec!["a", "d"]),
			Features::Some(vec!["b", "c"]),
			Features::Some(vec!["b", "d"]),
			Features::Some(vec!["c", "d"]),
			Features::Some(vec!["b", "c", "d"]),
			Features::All,
		];

		assert_eq!(combinations, expected);
	}
}
