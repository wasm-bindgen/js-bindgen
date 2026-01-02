js_bindgen::unsafe_embed_asm!();
//~^ ERROR: requires at least a string argument

js_bindgen::unsafe_embed_asm!(42);
//~^ ERROR: requires at least a string argument

js_bindgen::unsafe_embed_asm!("", "\r");
//~^ ERROR: escaping `r` is not supported

js_bindgen::unsafe_embed_asm!("" 42);
//~^ ERROR: expected a `,` after string literal

js_bindgen::unsafe_embed_asm!(.);
//~^ ERROR: requires at least a string argument

js_bindgen::unsafe_embed_asm!(#);
//~^ ERROR: expected `#[...]`

js_bindgen::unsafe_embed_asm!(#foo);
//~^ ERROR: expected `#[...]`

js_bindgen::unsafe_embed_asm!(#());
//~^ ERROR: expected `#[...]`

js_bindgen::unsafe_embed_asm!(#[foo]);
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_embed_asm!(
	#[cfg()]
	#[cfg()]
	""
);
//~^^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_embed_asm!(#[cfg()]);
//~^ ERROR: requires at least a string argument

js_bindgen::unsafe_embed_asm!("{}}");
//~^ ERROR: no corresponding closing bracers found

js_bindgen::unsafe_embed_asm!("{}", foo);
//~^ ERROR: expected `interpolate`

js_bindgen::unsafe_embed_asm!("{}", interpolate);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", interpolate *mut);
//~^ ERROR: expected `*const`

js_bindgen::unsafe_embed_asm!("{}", interpolate #);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", interpolate < Foo);
//~^ ERROR: type not completed, missing `>`

js_bindgen::unsafe_embed_asm!("{}", interpolate,);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", interpolate Foo &);
//~^ ERROR: expected a `,` between formatting parameters

js_bindgen::unsafe_embed_asm!("foo", "{}");
//~^ ERROR: expected an argument for `{}`

js_bindgen::unsafe_embed_asm!("foo", "{}", "{}", interpolate "test",);
//~^ ERROR: expected an argument for `{}`

js_bindgen::unsafe_embed_asm!("foo", "{a");
//~^ ERROR: no corresponding closing bracers found

js_bindgen::unsafe_embed_asm!("foo", "a}");
//~^ ERROR: no corresponding opening bracers found

js_bindgen::unsafe_embed_asm!("", 42);
//~^ ERROR: expected no tokens after string literals and formatting parameters
