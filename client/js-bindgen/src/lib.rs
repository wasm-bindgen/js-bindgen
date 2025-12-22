#[cfg(feature = "bootstrap")]
mod bootstrap;
mod cache;

use std::env;
use std::path::PathBuf;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn cache_embed_asm(input: TokenStream) -> TokenStream {
	let library = Library::new();

	if env::var_os(format!("JS_BINDGEN_BOOTSTRAP_{}", library.package))
		.filter(|value| value == "1")
		.is_some()
	{
		#[cfg(feature = "bootstrap")]
		{
			match bootstrap::run(input, library) {
				Ok(output) => output,
				Err(error) => error.into_compile_error().into(),
			}
		}
		#[cfg(not(feature = "bootstrap"))]
		return cache::compile_error(
			Span::mixed_site(),
			format!(
				"enabled bootstrap mode via environment variable for `{}` v{} but without \
				 enabling the `bootstrap` crate feature",
				library.package, library.version
			),
		);
	} else {
		cache::run(input, library).unwrap_or_else(|e| e)
	}
}

struct Library {
	package: String,
	version: String,
}

impl Library {
	fn new() -> Self {
		Self {
			package: env::var("CARGO_PKG_NAME").expect("`CARGO_PKG_NAME` should be present"),
			version: env::var("CARGO_PKG_VERSION").expect("`CARGO_PKG_VERSION` should be present"),
		}
	}

	fn dir(&self) -> PathBuf {
		let env = format!("JS_BINDGEN_CACHE_DIR_{}_{}", self.package, self.version);
		let dir = env::var_os(&env).unwrap_or_else(|| panic!("`{env}` should be present"));
		PathBuf::from(dir)
	}

	fn file(&self, name: &str) -> String {
		format!("{}.{}.{name}", self.package, self.version)
	}
}

/// Generates the output to import the archive.
///
/// ```
/// #[link(name = "...", kind = "static")]
/// extern "C" {}
/// ```
fn output(span: Span, library_file_name: &str) -> TokenStream {
	TokenStream::from_iter([
		// #[...]
		TokenTree::Punct(Punct::new('#', Spacing::Alone)),
		TokenTree::Group(Group::new(
			Delimiter::Bracket,
			// #[link(...)]
			TokenStream::from_iter([
				TokenTree::Ident(Ident::new("link", span)),
				TokenTree::Group(Group::new(
					Delimiter::Parenthesis,
					TokenStream::from_iter([
						// name = "library"
						TokenTree::Ident(Ident::new("name", span)),
						TokenTree::Punct(Punct::new('=', Spacing::Alone)),
						TokenTree::Literal(Literal::string(library_file_name)),
						// kind = "static"
						TokenTree::Punct(Punct::new(',', Spacing::Alone)),
						TokenTree::Ident(Ident::new("kind", span)),
						TokenTree::Punct(Punct::new('=', Spacing::Alone)),
						TokenTree::Literal(Literal::string("static")),
					]),
				)),
			]),
		)),
		// extern "C" { }
		TokenTree::Ident(Ident::new("extern", span)),
		TokenTree::Literal(Literal::string("C")),
		TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::new())),
	])
}
