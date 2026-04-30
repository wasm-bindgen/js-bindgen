macro_rules! enum_with_all {
	($vis:vis enum $name:ident, $variant:ident($type:ident), $opt:literal) => {
		paste::paste! {
			#[derive(Clone, Copy, Eq, PartialEq)]
			$vis enum $name {
				All,
				$variant($type),
			}

			const _: () = {
				use std::iter;
				use std::sync::LazyLock;

				use anyhow::{anyhow, Result};
				use clap::ValueEnum;
				use clap::builder::PossibleValue;
				use strum::IntoEnumIterator;

				impl Default for $name {
					fn default() -> Self {
						Self::$variant($type::default())
					}
				}

				impl ValueEnum for $name {
					fn value_variants<'a>() -> &'a [Self] {
						static VARIANTS: LazyLock<Vec<$name>> = LazyLock::new(|| {
							iter::once($name::All)
								.chain($type::iter().map($name::$variant))
								.collect()
						});

						&VARIANTS
					}

					fn to_possible_value(&self) -> Option<PossibleValue> {
						match self {
							Self::All => Some(PossibleValue::new("all")),
							Self::$variant(value) => value.to_possible_value(),
						}
					}
				}

				impl $name {
					#[allow(clippy::allow_attributes, unused, reason = "not always used")]
					$vis fn all() -> Vec<Self> {
						vec![Self::All]
					}

					$vis fn default_arg() -> &'static str {
						static DEFAULT: LazyLock<String> = LazyLock::new(|| {
							let value = $name::default().to_possible_value().unwrap();
							value.get_name().to_owned()
						});

						&DEFAULT
					}
				}

				impl $type {
					$vis fn [<from_ $opt>](cli: Vec<$name>) -> Result<Vec<Self>> {
						if let [$name::All] = cli.as_slice() {
							return Ok(Self::iter().collect());
						}

						cli.into_iter()
							.map(|runner| match runner {
								$name::All => Err(anyhow!(
									"`--{}`s `all` option conflicts with all others", $opt
								)),
								$name::$variant(value) => Ok(value),
							})
							.collect()
					}
				}
			};
		}
	};
}
