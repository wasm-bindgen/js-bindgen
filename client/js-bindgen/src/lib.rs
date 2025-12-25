use std::borrow::Cow;
use std::env;
use std::fmt::Display;
use std::iter::Peekable;

use proc_macro::{
	token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[proc_macro]
pub fn embed_asm(input: TokenStream) -> TokenStream {
	let span = Span::mixed_site();
	let mut assembly = match parse_string_list(span, input.into_iter().peekable()) {
		Ok(assembly) => assembly,
		Err(error) => return error,
	};
	assembly.push('\0');

	custom_section(span, "js_bindgen.assembly", assembly.as_bytes())
}

#[proc_macro]
pub fn js_import(input: TokenStream) -> TokenStream {
	let span = Span::mixed_site();
	let mut input = input.into_iter().peekable();

	match input.next() {
		Some(TokenTree::Ident(i)) if i.to_string() == "name" => (),
		Some(token) => return compile_error(token.span(), "expected `name = \"...\"`"),
		None => return compile_error(span, "expected `name = \"...\"`"),
	}

	match input.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
		Some(token) => return compile_error(token.span(), "expected `= \"...\"`"),
		None => return compile_error(span, "expected `= \"...\"`"),
	}

	let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
	let name = match input.next() {
		Some(TokenTree::Literal(l)) => {
			let span = l.span();
			let literal = l.to_string();
			match parse_string_literal(span, &literal) {
				Ok(string) => string.into_owned(),
				Err(error) => return error,
			}
		}
		Some(token) => return compile_error(token.span(), "expected a string literal"),
		None => return compile_error(span, "expected a string literal`"),
	};

	match input.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == ',' => (),
		Some(token) => {
			return compile_error(token.span(), "expected `,` and a list of string literals")
		}
		None => return compile_error(span, "expected a list of string literals"),
	}

	let mut assembly = match parse_string_list(span, input) {
		Ok(assembly) => assembly,
		Err(error) => return error,
	};
	assembly.push('\0');

	custom_section(
		span,
		&format!("js_bindgen.import.{package}.{name}"),
		assembly.as_bytes(),
	)
}

fn parse_string_list(
	span: Span,
	input: Peekable<token_stream::IntoIter>,
) -> Result<String, TokenStream> {
	let mut assembly = String::new();
	let mut input = input.into_iter().peekable();

	while let Some(token) = input.next() {
		match token {
			TokenTree::Literal(lit) => {
				let span = lit.span();
				let lit = lit.to_string();
				let string = parse_string_literal(span, &lit)?;

				// Only insert newline when there are multiple strings.
				if assembly.is_empty() {
					assembly = string.into_owned();
				} else {
					assembly.extend([Cow::Borrowed("\n"), string]);
				}

				match input.peek() {
					Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
						input.next();
					}
					Some(token) => return Err(compile_error(token.span(), "expecting a `,`")),
					None => (),
				}
			}
			token => return Err(compile_error(token.span(), "expecting a string literal")),
		}
	}

	if assembly.is_empty() {
		Err(compile_error(span, "requires at least a string argument"))
	} else {
		Ok(assembly)
	}
}

fn parse_string_literal(span: Span, lit: &str) -> Result<Cow<'_, str>, TokenStream> {
	// Strip starting and ending `"`.
	let Some(stripped) = lit.strip_prefix('"').and_then(|lit| lit.strip_suffix('"')) else {
		return Err(compile_error(span, "expecting a string literal"));
	};

	// Apply escaping `\`.
	let sanitized = if stripped.contains("\\\\") {
		Cow::Owned(stripped.replace("\\\\", "\\"))
	} else {
		Cow::Borrowed(stripped)
	};
	// Apply escaping `"`.
	let sanitized = if stripped.contains("\\\"") {
		Cow::Owned(sanitized.replace("\\\"", "\""))
	} else {
		sanitized
	};

	// Don't allow escaping anything else.
	if sanitized.contains('\\') {
		return Err(compile_error(span, "only escaping `\"` is supported"));
	}

	// Don't allow null characters, as we use those as delimiters between assembly
	// blocks.
	if sanitized.contains('\0') {
		return Err(compile_error(span, " null characters are not supported"));
	}

	Ok(sanitized)
}

fn custom_section(span: Span, name: &str, data: &[u8]) -> TokenStream {
	TokenStream::from_iter([
		TokenTree::Ident(Ident::new("const", span)),
		TokenTree::Ident(Ident::new("_", span)),
		TokenTree::Punct(Punct::new(':', Spacing::Alone)),
		TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
		TokenTree::Punct(Punct::new('=', Spacing::Alone)),
		TokenTree::Group(Group::new(
			Delimiter::Brace,
			TokenStream::from_iter([
				TokenTree::Punct(Punct::new('#', Spacing::Alone)),
				TokenTree::Group(Group::new(
					Delimiter::Bracket,
					TokenStream::from_iter([
						TokenTree::Ident(Ident::new("link_section", span)),
						TokenTree::Punct(Punct::new('=', Spacing::Alone)),
						TokenTree::Literal(Literal::string(name)),
					]),
				)),
				TokenTree::Ident(Ident::new("static", span)),
				TokenTree::Ident(Ident::new("CUSTOM_SECTION", span)),
				TokenTree::Punct(Punct::new(':', Spacing::Alone)),
				TokenTree::Group(Group::new(
					Delimiter::Bracket,
					TokenStream::from_iter([
						TokenTree::Ident(Ident::new("u8", span)),
						TokenTree::Punct(Punct::new(';', Spacing::Alone)),
						TokenTree::Literal(Literal::usize_unsuffixed(data.len())),
					]),
				)),
				TokenTree::Punct(Punct::new('=', Spacing::Alone)),
				TokenTree::Punct(Punct::new('*', Spacing::Alone)),
				TokenTree::Literal(Literal::byte_string(data)),
				TokenTree::Punct(Punct::new(';', Spacing::Alone)),
			]),
		)),
		TokenTree::Punct(Punct::new(';', Spacing::Alone)),
	])
}

fn compile_error<E: Display>(span: Span, error: E) -> TokenStream {
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
