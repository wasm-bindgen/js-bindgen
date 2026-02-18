use proc_macro2::Span;
use syn::{Attribute, File, Ident, Item, Path, Visibility, parse_quote};
use weedle::common::Docstring;
use weedle::{Definition, Err, Error, InterfaceDefinition};

use crate::{Hygiene, ImportManager, Type};

pub fn from_web_idl<'i>(
	web_idl: &'i str,
	js_sys: Option<Path>,
	vis: &Visibility,
) -> Result<File, Err<Error<&'i str>>> {
	let mut imports = ImportManager::new(js_sys);
	let mut items: Vec<Item> = Vec::new();

	for definition in weedle::parse(web_idl)? {
		match definition {
			Definition::Interface(InterfaceDefinition {
				docstring,
				attributes,
				identifier,
				inheritance,
				members,
				..
			}) => {
				let attrs: &[Attribute] = if let Some(Docstring(docstring)) = docstring {
					&[parse_quote!(#[doc = #docstring])]
				} else {
					&[]
				};

				if attributes.is_some() {
					todo!()
				}

				let identifier = Ident::new(identifier.0, Span::mixed_site());
				items.extend(Type::new(
					Hygiene::Imports(&mut imports),
					parse_quote!(#(#attrs)* #vis type #identifier;),
				));

				if inheritance.is_some() {
					todo!()
				}

				if !members.body.is_empty() {
					todo!()
				}
			}
			_ => todo!(),
		}
	}

	let items = imports.iter().map(Item::from).chain(items).collect();

	Ok(File {
		shebang: None,
		attrs: Vec::new(),
		items,
	})
}
