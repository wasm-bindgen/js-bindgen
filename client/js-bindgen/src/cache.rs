use std::fmt::Display;
use std::io::ErrorKind;
use std::{env, fs};

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::Library;

pub(crate) fn run(input: TokenStream, library: Library) -> Result<TokenStream, TokenStream> {
	let span = Span::mixed_site();
	let mut input = input.into_iter();

	let name = match input.next() {
		Some(TokenTree::Ident(ident)) if ident.to_string() == "name" => ident,
		Some(tok) => return Err(compile_error(tok.span(), "expected `name`")),
		_ => return Err(compile_error(span, "expected `name`")),
	};

	let equal = match input.next() {
		Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => punct,
		Some(tok) => return Err(compile_error(tok.span(), "expected `=`")),
		_ => return Err(compile_error(name.span(), "expected `=` after `name`")),
	};

	let name = match input.next() {
		Some(TokenTree::Literal(lit)) => {
			let text = lit.to_string();

			if let Some(stripped) = text.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
				stripped.to_owned()
			} else {
				return Err(compile_error(lit.span(), "expected string literal"));
			}
		}
		Some(tok) => return Err(compile_error(tok.span(), "expected string literal")),
		_ => {
			return Err(compile_error(
				equal.span(),
				"expected string literal after `=`",
			))
		}
	};

	// For Rust Analyzer we just want parse errors, the rest doesn't work.
	if env::var_os("RUST_ANALYZER_INTERNALS_DO_NOT_USE")
		.filter(|value| value == "this is unstable")
		.is_some()
	{
		return Ok(TokenStream::new());
	}

	let library_file = library.file(&name.to_string());

	if env::var_os("JS_BINDGEN_CACHE_DISABLE_CHECK")
		.filter(|value| value == "1")
		.is_none()
	{
		let source_path = span
			.local_file()
			.ok_or_else(|| compile_error(span, "unable to get path to source file"))?;
		let archive_path = library
			.dir()
			.join(format!("lib{library_file}"))
			.with_added_extension("a");

		let source_mtime = fs::metadata(source_path)
			.and_then(|m| m.modified())
			.map_err(|e| compile_error(span, e))?;

		let archive_mtime = match fs::metadata(archive_path) {
			Ok(m) => m.modified().map_err(|e| compile_error(span, e))?,
			Err(e) if matches!(e.kind(), ErrorKind::NotFound) => {
				return Err(compile_error(
					span,
					format!(
						"archive `{name}` was not found in `{}` v{}, contact the authors to \
						 resolve this",
						library.package, library.version
					),
				))
			}
			Err(e) => return Err(compile_error(span, e)),
		};

		if archive_mtime != source_mtime {
			return Err(compile_error(span, "archive is not up-to-date"));
		}
	}

	Ok(crate::output(span, &library_file))
}

pub(crate) fn compile_error<E: Display>(span: Span, error: E) -> TokenStream {
	TokenStream::from_iter([
		TokenTree::Punct(Punct::new(':', Spacing::Joint)),
		TokenTree::Punct(Punct::new(':', Spacing::Alone)),
		TokenTree::Ident(Ident::new("core", span)),
		TokenTree::Punct(Punct::new(':', Spacing::Joint)),
		TokenTree::Punct(Punct::new(':', Spacing::Alone)),
		TokenTree::Ident(Ident::new("compile_error", span)),
		TokenTree::Punct(Punct::new('!', Spacing::Alone)),
		TokenTree::Group(Group::new(
			Delimiter::Parenthesis,
			TokenTree::Literal(Literal::string(&error.to_string())).into(),
		)),
		TokenTree::Punct(Punct::new(';', Spacing::Alone)),
	])
}
