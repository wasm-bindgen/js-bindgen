js_bindgen::unsafe_embed_asm!();
//~^ ERROR: requires at least a string template

js_bindgen::unsafe_embed_asm!(42);
//~^ ERROR: requires at least a string template

js_bindgen::unsafe_embed_asm!("\r");
//~^ ERROR: escaping `r` is not supported

js_bindgen::unsafe_embed_asm!("" 42);
//~^ ERROR: expected a `,` after string literal

js_bindgen::unsafe_embed_asm!(.);
//~^ ERROR: requires at least a string template

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
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_embed_asm!(#[test]);
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_embed_asm!(#[cfg()]);
//~^ ERROR: leftover `cfg` attribute

js_bindgen::unsafe_embed_asm!("{}}");
//~^ ERROR: no corresponding closing bracers found

js_bindgen::unsafe_embed_asm!("{}", foo);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", interpolate);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", foo "test");
//~^ ERROR: expected `const` or `interpolate`

js_bindgen::unsafe_embed_asm!("{}", interpolate #);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", interpolate < Foo);
//~^ ERROR: type not completed, missing `>`

js_bindgen::unsafe_embed_asm!("{}", interpolate,);
//~^ ERROR: expected a value

js_bindgen::unsafe_embed_asm!("{}", interpolate Foo $);
//~^ ERROR: expected a `,` between formatting parameters

js_bindgen::unsafe_embed_asm!("{}");
//~^ ERROR: expected an argument for `{}`

js_bindgen::unsafe_embed_asm!("{}", "{}", interpolate "test",);
//~^ ERROR: expected an argument for `{}`

js_bindgen::unsafe_embed_asm!("{");
//~^ ERROR: no corresponding closing bracers found

js_bindgen::unsafe_embed_asm!("}");
//~^ ERROR: no corresponding opening bracers found

js_bindgen::unsafe_embed_asm!("", 42);
//~^ ERROR: expected named argument, `const` or `interpolate`

js_bindgen::unsafe_embed_asm!("", #[] );
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_embed_asm!("", #[cfg()] );
//~^ ERROR: leftover `cfg` attribute

js_bindgen::unsafe_embed_asm!("", #[cfg()] $ );
//~^ ERROR: expected named argument, `const` or `interpolate`

js_bindgen::unsafe_embed_asm!(
	"",
	#[cfg()]
	#[test]
);
//~^^ ERROR: expected `cfg`

js_bindgen::unsafe_embed_asm!(
	"",
	#[cfg()]
	#[cfg()]
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_embed_asm!("{}", interpolate "test", #[test] );
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_embed_asm!("{}", interpolate "test", #[cfg()] );
//~^ ERROR: leftover `cfg` attribute

js_bindgen::unsafe_embed_asm!("{}", interpolate "test", #[cfg()] $ );
//~^ ERROR: expected named argument, `const` or `interpolate`

js_bindgen::unsafe_embed_asm!(
	"{}",
	interpolate "test",
	#[cfg()]
	#[test]
);
//~^^ ERROR: expected `cfg`

js_bindgen::unsafe_embed_asm!(
	"{}",
	interpolate "test",
	#[cfg()]
	#[cfg()]
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_embed_asm!("{}", par = "test");
//~^ ERROR: expected `const` or `interpolate`

js_bindgen::unsafe_embed_asm!("{}", par = operator "test");
//~^ ERROR: expected `const` or `interpolate`

js_bindgen::unsafe_embed_asm!("{}", interpolate "test", par = interpolate "test");
//~^ ERROR: named argument must come first

js_bindgen::unsafe_embed_asm!("{par}", par = interpolate "test 1", par = interpolate "test 2");
//~^ ERROR: found duplicate named argument

js_bindgen::unsafe_embed_asm!("{par}", #[cfg()] par = interpolate "test 1", #[cfg()] par = interpolate "test 2");
//~^ ERROR: found duplicate named argument

js_bindgen::unsafe_embed_asm!("{}", #[cfg()] interpolate "test 1");
//~^ ERROR: `cfg` attributes are only supported on named arguments

js_bindgen::unsafe_embed_asm!("{()}");
//~^ ERROR: template string named argument identifier

js_bindgen::unsafe_embed_asm!("{par()}");
//~^ ERROR: template string named argument identifier

js_bindgen::unsafe_embed_asm!("{[}");
//~^ ERROR: invalid template string named argument identifier
//~^^ ERROR: this file contains an unclosed delimiter

js_bindgen::unsafe_embed_asm!("{par}");
//~^ ERROR: expected a named argument for `par`

js_bindgen::unsafe_embed_asm!("", interpolate "test");
//~^ ERROR: expected no leftover arguments

js_bindgen::unsafe_embed_asm!("", par = interpolate "test");
//~^ ERROR: expected no leftover arguments
