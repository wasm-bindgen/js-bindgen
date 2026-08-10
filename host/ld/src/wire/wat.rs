//! Shared WAT emission helpers.

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter, Write};

use js_bindgen_wire::abi::WatType;
use js_bindgen_wire::model::{WatImport, WatImportKind, WatLocal};

/// Formats a quoted WAT string as UTF-8 bytes.
///
/// Byte escapes keep arbitrary Unicode and control characters independent of
/// how the WAT source text is parsed.
pub(super) fn quoted(value: &str) -> impl Display + '_ {
	Quoted(value)
}

struct Quoted<'a>(&'a str);

impl Display for Quoted<'_> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		formatter.write_char('"')?;
		for &byte in self.0.as_bytes() {
			match byte {
				b'"' => formatter.write_str("\\\"")?,
				b'\\' => formatter.write_str("\\\\")?,
				0x20..=0x7e => formatter.write_char(char::from(byte))?,
				byte => write!(formatter, "\\{byte:02x}")?,
			}
		}
		formatter.write_char('"')
	}
}

#[derive(Default)]
pub(super) struct WatImports {
	indices: HashMap<String, usize>,
	entries: Vec<String>,
}

impl WatImports {
	pub(super) fn extend(&mut self, imports: &[WatImport<'_>]) {
		for import in imports {
			let mut wat = String::new();
			write_import(&mut wat, import);
			self.insert(import.identifier, wat);
		}
	}

	pub(super) fn insert(&mut self, identifier: &str, wat: String) {
		if let Some(&index) = self.indices.get(identifier) {
			assert_eq!(
				self.entries[index], wat,
				"conflicting WAT imports use `${identifier}`",
			);
		} else {
			self.indices
				.insert(identifier.to_owned(), self.entries.len());
			self.entries.push(wat);
		}
	}

	pub(super) fn render(self) -> String {
		self.entries.join("\n")
	}
}

#[derive(Default)]
pub(super) struct WatLocals<'wire> {
	entries: Vec<WatLocal<'wire>>,
}

impl<'wire> WatLocals<'wire> {
	pub(super) fn extend(&mut self, locals: &[WatLocal<'wire>]) {
		for &local in locals {
			if let Some(existing) = self
				.entries
				.iter()
				.find(|existing| existing.name == local.name)
			{
				assert_eq!(
					existing, &local,
					"conflicting WAT locals use `${}`",
					local.name,
				);
			} else {
				self.entries.push(local);
			}
		}
	}

	pub(super) fn write_into(self, wat: &mut String) {
		for local in self.entries {
			write!(wat, "\n  (local ${} {})", local.name, local.ty)
				.expect("writing to a String cannot fail");
		}
	}
}

pub(super) fn write_conversion(wat: &mut String, conversion: Option<&str>) {
	if let Some(conversion) = conversion
		&& !conversion.is_empty()
	{
		wat.push_str("\n  ");
		wat.push_str(conversion);
	}
}

fn write_import(wat: &mut String, import: &WatImport<'_>) {
	write!(
		wat,
		"(import {} {} (",
		quoted(import.module),
		quoted(import.name),
	)
	.expect("writing to a String cannot fail");

	match &import.kind {
		WatImportKind::Function {
			parameters,
			results,
		} => {
			write!(wat, "func ${} ", import.identifier).expect("writing to a String cannot fail");
			write_symbol(wat, import.symbol_name);
			write_types(wat, "param", parameters);
			write_types(wat, "result", results);
		}
		WatImportKind::Table {
			index_type,
			minimum,
			maximum,
			element,
		} => {
			write!(wat, "table ${} ", import.identifier).expect("writing to a String cannot fail");
			write_symbol(wat, import.symbol_name);
			write!(wat, " {}{minimum}", index_type.wat_prefix())
				.expect("writing to a String cannot fail");
			if let Some(maximum) = maximum {
				write!(wat, " {maximum}").expect("writing to a String cannot fail");
			}
			write!(wat, " {element}").expect("writing to a String cannot fail");
		}
		WatImportKind::Tag { parameters } => {
			write!(wat, "tag ${} ", import.identifier).expect("writing to a String cannot fail");
			write_symbol(wat, import.symbol_name);
			write_types(wat, "param", parameters);
		}
	}

	wat.push_str("))");
}

fn write_symbol(wat: &mut String, name: Option<&str>) {
	if let Some(name) = name {
		write!(wat, "(@sym (name {}))", quoted(name)).expect("writing to a String cannot fail");
	} else {
		wat.push_str("(@sym)");
	}
}

fn write_types(wat: &mut String, kind: &str, types: &[WatType]) {
	if types.is_empty() {
		return;
	}
	write!(wat, " ({kind}").expect("writing to a String cannot fail");
	for ty in types {
		write!(wat, " {ty}").expect("writing to a String cannot fail");
	}
	wat.push(')');
}

#[cfg(test)]
mod tests {
	use super::quoted;

	#[test]
	fn quotes_wat_strings_as_bytes() {
		assert_eq!(
			quoted("single' double\" slash\\ line\n雪").to_string(),
			"\"single' double\\\" slash\\\\ line\\0a\\e9\\9b\\aa\"",
		);
	}
}
