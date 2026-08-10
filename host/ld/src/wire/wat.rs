//! Shared WAT emission helpers.

use std::collections::HashMap;
use std::fmt::Write;

use js_bindgen_wire::abi::WatType;
use js_bindgen_wire::model::{WatImport, WatImportKind, WatLocal};

/// Quotes a WAT string as UTF-8 bytes.
///
/// Byte escapes keep arbitrary Unicode and control characters independent of
/// how the WAT source text is parsed.
pub(super) fn quote_string(value: &str) -> String {
	let mut output = String::with_capacity(value.len() + 2);
	output.push('"');
	for &byte in value.as_bytes() {
		match byte {
			b'"' => output.push_str("\\\""),
			b'\\' => output.push_str("\\\\"),
			0x20..=0x7e => output.push(char::from(byte)),
			byte => write!(output, "\\{byte:02x}").expect("writing to a String cannot fail"),
		}
	}
	output.push('"');
	output
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
		let mut wat = String::new();
		for entry in self.entries {
			write_separator(&mut wat);
			wat.push_str(&entry);
		}
		wat
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

	pub(super) fn render(self) -> String {
		let mut wat = String::new();
		for local in self.entries {
			write_separator(&mut wat);
			write!(wat, "  (local ${} {})", local.name, local.ty)
				.expect("writing to a String cannot fail");
		}
		wat
	}
}

pub(super) fn write_conversion(wat: &mut String, conversion: &str) {
	if !conversion.is_empty() {
		wat.push_str("\n  ");
		wat.push_str(conversion);
	}
}

fn write_import(wat: &mut String, import: &WatImport<'_>) {
	write!(
		wat,
		"(import {} {} (",
		quote_string(import.module),
		quote_string(import.name),
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
		write!(wat, "(@sym (name {}))", quote_string(name))
			.expect("writing to a String cannot fail");
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

fn write_separator(wat: &mut String) {
	if !wat.is_empty() {
		wat.push('\n');
	}
}

#[cfg(test)]
mod tests {
	use super::quote_string;

	#[test]
	fn quotes_wat_strings_as_bytes() {
		assert_eq!(
			quote_string("single' double\" slash\\ line\n雪"),
			"\"single' double\\\" slash\\\\ line\\0a\\e9\\9b\\aa\"",
		);
	}
}
