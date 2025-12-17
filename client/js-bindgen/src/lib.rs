#![feature(allow_internal_unstable, asm_experimental_arch)]
#![allow(internal_features)]

use proc_macro::Delimiter;
use proc_macro::Group;
use proc_macro::Ident;
use proc_macro::Punct;
use proc_macro::Spacing;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro]
#[allow_internal_unstable(asm_experimental_arch)]
pub fn global_asm(input: TokenStream) -> TokenStream {
    let span = Span::call_site();
    let mut out = TokenStream::new();

    // ::core
    out.extend([TokenTree::Punct(Punct::new(':', Spacing::Joint))]);
    out.extend([TokenTree::Punct(Punct::new(':', Spacing::Alone))]);
    out.extend([TokenTree::Ident(Ident::new("core", span))]);
    // ::arch
    out.extend([TokenTree::Punct(Punct::new(':', Spacing::Joint))]);
    out.extend([TokenTree::Punct(Punct::new(':', Spacing::Alone))]);
    out.extend([TokenTree::Ident(Ident::new("arch", span))]);
    // ::global_asm!
    out.extend([TokenTree::Punct(Punct::new(':', Spacing::Joint))]);
    out.extend([TokenTree::Punct(Punct::new(':', Spacing::Alone))]);
    out.extend([TokenTree::Ident(Ident::new("global_asm", span))]);
    out.extend([TokenTree::Punct(Punct::new('!', Spacing::Alone))]);
    // original input
    out.extend([TokenTree::Group(Group::new(Delimiter::Brace, input))]);

    out
}
