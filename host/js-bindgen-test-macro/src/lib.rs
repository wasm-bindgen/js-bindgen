use std::iter::{self, Peekable};

use js_bindgen_macro_shared::*;
use proc_macro::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};

struct TestAttributes {
	ignore: Option<Option<String>>,
	should_panic: Option<Option<String>>,
}

impl TestAttributes {
	fn new() -> Self {
		Self {
			ignore: None,
			should_panic: None,
		}
	}
}

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
	test_internal(attr, item).unwrap_or_else(|e| e)
}

fn test_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut attr = attr.into_iter();
	if let Some(tok) = attr.next() {
		return Err(compile_error(tok.span(), "expected empty attribute"));
	}

	let (item, attrs) = strip_test_attributes(item)?;
	let (ident, is_async) = find_test_ident(&item)?;
	if is_async {
		return Err(compile_error(ident.span(), "async tests are not supported"));
	}

	let test_name_tokens = test_name_tokens(&ident);

	let section_data = [
		Argument {
			cfg: None,
			kind: ArgumentKind::Bytes(option_option_string(&attrs.ignore)),
		},
		Argument {
			cfg: None,
			kind: ArgumentKind::Bytes(option_option_string(&attrs.should_panic)),
		},
		Argument {
			cfg: None,
			kind: ArgumentKind::Interpolate(test_name_tokens),
		},
	];

	let mut output = TokenStream::new();
	output.extend(item);

	let section = custom_section("js_bindgen.test", None, &section_data);
	output.extend(section);

	let wrapper = format!(
		r#"
const _: () = {{
    #[unsafe(export_name = ::core::concat!(::core::module_path!(), "::", ::core::stringify!({ident})))]
    extern "C" fn jbg_test() {{
        js_bindgen_test::set_panic_hook();
	    {ident}();
    }}
}};
"#
	);
	output.extend(wrapper.parse::<TokenStream>().unwrap());

	Ok(output)
}

fn option_option_string(option: &Option<Option<String>>) -> Vec<u8> {
	match option {
		Some(s) => {
			if let Some(s) = s {
				let len = u32::to_le_bytes(s.len() as u32);
				[2].iter()
					.chain(&len)
					.chain(s.as_bytes())
					.copied()
					.collect::<Vec<_>>()
			} else {
				vec![1]
			}
		}
		None => vec![0],
	}
}

fn find_test_ident(item: &TokenStream) -> Result<(Ident, bool), TokenStream> {
	let mut iter = item.clone().into_iter().peekable();
	let mut saw_async = false;
	let mut last_span = Span::mixed_site();

	while let Some(tok) = iter.next() {
		last_span = tok.span();
		if let TokenTree::Ident(ident) = &tok {
			let name = ident.to_string();
			if name == "async" {
				saw_async = true;
			}
			if name == "fn" {
				let ident = parse_ident(&mut iter, ident.span(), "a function name")?;
				return Ok((ident, saw_async));
			}
		}
	}

	Err(compile_error(last_span, "expected a function"))
}

enum TestAttribute {
	Ignore(Option<String>),
	ShouldPanic(Option<String>),
}

fn strip_test_attributes(item: TokenStream) -> Result<(TokenStream, TestAttributes), TokenStream> {
	let mut iter = item.into_iter().peekable();
	let mut output = Vec::new();
	let mut attrs = TestAttributes::new();

	while let Some(tok) = iter.next() {
		let TokenTree::Punct(punct) = &tok else {
			output.push(tok);
			continue;
		};

		if punct.as_char() != '#' {
			output.push(tok);
			continue;
		}

		let Some(TokenTree::Group(group)) = iter.peek() else {
			output.push(tok);
			continue;
		};
		if group.delimiter() != Delimiter::Bracket {
			output.push(tok);
			continue;
		}

		match parse_test_attribute(group)? {
			Some(TestAttribute::Ignore(reason)) => {
				if attrs.ignore.replace(reason).is_some() {
					return Err(compile_error(group.span(), "duplicate `ignore` attribute"));
				}
				iter.next();
			}
			Some(TestAttribute::ShouldPanic(message)) => {
				if attrs.should_panic.replace(message).is_some() {
					return Err(compile_error(
						group.span(),
						"duplicate `should_panic` attribute",
					));
				}
				iter.next();
			}
			None => {
				output.push(tok);
				output.push(TokenTree::Group(group.clone()));
				iter.next();
			}
		}
	}

	Ok((TokenStream::from_iter(output), attrs))
}

fn parse_test_attribute(group: &Group) -> Result<Option<TestAttribute>, TokenStream> {
	let mut stream = group.stream().into_iter().peekable();
	let Some(TokenTree::Ident(ident)) = stream.next() else {
		return Ok(None);
	};

	match ident.to_string().as_str() {
		"ignore" => {
			let reason = parse_optional_reason(&mut stream, ident.span())?;
			Ok(Some(TestAttribute::Ignore(reason)))
		}
		"should_panic" => {
			let reason = parse_should_panic_reason(&mut stream, ident.span())?;
			Ok(Some(TestAttribute::ShouldPanic(reason)))
		}
		_ => Ok(None),
	}
}

fn parse_optional_reason(
	stream: &mut Peekable<token_stream::IntoIter>,
	span: Span,
) -> Result<Option<String>, TokenStream> {
	if let Some(TokenTree::Punct(punct)) = stream.peek() {
		if punct.as_char() == '=' {
			let punct = expect_punct(&mut *stream, '=', span, "`=`", false)?;
			let (_, reason) =
				parse_string_literal(&mut *stream, punct.span(), "a string literal", false)?;
			if stream.peek().is_some() {
				return Err(compile_error(span, "unexpected tokens"));
			}
			return Ok(Some(reason));
		}
	}
	if stream.peek().is_some() {
		return Err(compile_error(span, "unexpected tokens"));
	}
	Ok(None)
}

fn parse_should_panic_reason(
	stream: &mut Peekable<token_stream::IntoIter>,
	span: Span,
) -> Result<Option<String>, TokenStream> {
	// Support `#[should_panic = "..."]` and `#[should_panic(expected = "...")]`.
	if let Some(TokenTree::Punct(punct)) = stream.peek() {
		if punct.as_char() == '=' {
			let punct = expect_punct(&mut *stream, '=', span, "`=`", false)?;
			let (_, reason) =
				parse_string_literal(&mut *stream, punct.span(), "a string literal", false)?;
			if stream.peek().is_some() {
				return Err(compile_error(span, "unexpected tokens"));
			}
			return Ok(Some(reason));
		}
	}

	if let Some(TokenTree::Group(group)) = stream.peek() {
		if group.delimiter() != Delimiter::Parenthesis {
			return Err(compile_error(span, "unexpected tokens"));
		}
		let group = match stream.next() {
			Some(TokenTree::Group(group)) => group,
			_ => return Err(compile_error(span, "unexpected tokens")),
		};
		let mut inner = group.stream().into_iter().peekable();
		let Some(TokenTree::Ident(ident)) = inner.next() else {
			return Err(compile_error(group.span(), "expected `expected`"));
		};
		let name = ident.to_string();
		if name.as_str() != "expected" {
			return Err(compile_error(ident.span(), "expected `expected = \"...\"`"));
		}
		let punct = expect_punct(&mut inner, '=', ident.span(), "`=`", false)?;
		let (_, reason) =
			parse_string_literal(&mut inner, punct.span(), "a string literal", false)?;
		if inner.peek().is_some() || stream.peek().is_some() {
			return Err(compile_error(span, "unexpected tokens"));
		}
		return Ok(Some(reason));
	}

	if stream.peek().is_some() {
		return Err(compile_error(span, "unexpected tokens"));
	}

	Ok(None)
}

fn test_name_tokens(ident: &Ident) -> Vec<TokenTree> {
	let mut args = Vec::new();
	args.extend(path_tokens(&["core", "module_path"]));
	args.push(Punct::new('!', Spacing::Alone).into());
	args.push(group(Delimiter::Parenthesis, iter::empty()));
	args.push(Punct::new(',', Spacing::Alone).into());
	args.push(Literal::string("::").into());
	args.push(Punct::new(',', Spacing::Alone).into());
	args.extend(path_tokens(&["core", "stringify"]));
	args.push(Punct::new('!', Spacing::Alone).into());
	args.push(group(
		Delimiter::Parenthesis,
		iter::once(ident.clone().into()),
	));

	let mut concat = Vec::new();
	concat.extend(path_tokens(&["core", "concat"]));
	concat.push(Punct::new('!', Spacing::Alone).into());
	concat.push(group(Delimiter::Parenthesis, args));

	concat
}

fn path_tokens(segments: &[&str]) -> Vec<TokenTree> {
	let mut tokens = Vec::new();
	tokens.push(Punct::new(':', Spacing::Joint).into());
	tokens.push(Punct::new(':', Spacing::Alone).into());

	for (index, segment) in segments.iter().enumerate() {
		tokens.push(ident(segment));
		if index + 1 < segments.len() {
			tokens.push(Punct::new(':', Spacing::Joint).into());
			tokens.push(Punct::new(':', Spacing::Alone).into());
		}
	}

	tokens
}

fn group(delimiter: Delimiter, inner: impl IntoIterator<Item = TokenTree>) -> TokenTree {
	Group::new(delimiter, inner.into_iter().collect()).into()
}

fn ident(string: &str) -> TokenTree {
	Ident::new(string, Span::mixed_site()).into()
}
