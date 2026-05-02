js_bindgen::unsafe_global_wat!();
//~^ ERROR: requires at least a string template

js_bindgen::unsafe_global_wat!(42);
//~^ ERROR: requires at least a string template

js_bindgen::unsafe_global_wat!("\r");
//~^ ERROR: escaping `r` is not supported

js_bindgen::unsafe_global_wat!("" 42);
//~^ ERROR: expected a `,` after string literal

js_bindgen::unsafe_global_wat!(.);
//~^ ERROR: requires at least a string template

js_bindgen::unsafe_global_wat!(#);
//~^ ERROR: expected `#[...]`

js_bindgen::unsafe_global_wat!(#foo);
//~^ ERROR: expected `#[...]`

js_bindgen::unsafe_global_wat!(#());
//~^ ERROR: expected `#[...]`

js_bindgen::unsafe_global_wat!(#[foo]);
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_global_wat!(
	#[cfg()]
	#[cfg()]
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_global_wat!(#[test]);
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_global_wat!(#[cfg()]);
//~^ ERROR: leftover `cfg` attribute

js_bindgen::unsafe_global_wat!("{}}");
//~^ ERROR: no corresponding closing bracers found

js_bindgen::unsafe_global_wat!("{}", foo);
//~^ ERROR: expected a value

js_bindgen::unsafe_global_wat!("{}", interpolate);
//~^ ERROR: expected a value

js_bindgen::unsafe_global_wat!("{}", foo "test");
//~^ ERROR: expected `const` or `interpolate`

js_bindgen::unsafe_global_wat!("{}", interpolate #);
//~^ ERROR: expected a value

js_bindgen::unsafe_global_wat!("{}", interpolate < Foo);
//~^ ERROR: type not completed, missing `>`

js_bindgen::unsafe_global_wat!("{}", interpolate,);
//~^ ERROR: expected a value

js_bindgen::unsafe_global_wat!("{}", interpolate Foo $);
//~^ ERROR: expected a `,` between formatting parameters

js_bindgen::unsafe_global_wat!("{}");
//~^ ERROR: expected an argument for `{}`

js_bindgen::unsafe_global_wat!("{}", "{}", interpolate "test",);
//~^ ERROR: expected an argument for `{}`

js_bindgen::unsafe_global_wat!("{");
//~^ ERROR: no corresponding closing bracers found

js_bindgen::unsafe_global_wat!("}");
//~^ ERROR: no corresponding opening bracers found

js_bindgen::unsafe_global_wat!("", 42);
//~^ ERROR: expected named argument, `const` or `interpolate`

js_bindgen::unsafe_global_wat!("", #[] );
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_global_wat!("", #[cfg()] );
//~^ ERROR: leftover `cfg` attribute

js_bindgen::unsafe_global_wat!("", #[cfg()] $ );
//~^ ERROR: expected named argument, `const` or `interpolate`

js_bindgen::unsafe_global_wat!(
	"",
	#[cfg()]
	#[test]
);
//~^^ ERROR: expected `cfg`

js_bindgen::unsafe_global_wat!(
	"",
	#[cfg()]
	#[cfg()]
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_global_wat!("{}", interpolate "test", #[test] );
//~^ ERROR: expected `cfg`

js_bindgen::unsafe_global_wat!("{}", interpolate "test", #[cfg()] );
//~^ ERROR: leftover `cfg` attribute

js_bindgen::unsafe_global_wat!("{}", interpolate "test", #[cfg()] $ );
//~^ ERROR: expected named argument, `const` or `interpolate`

js_bindgen::unsafe_global_wat!(
	"{}",
	interpolate "test",
	#[cfg()]
	#[test]
);
//~^^ ERROR: expected `cfg`

js_bindgen::unsafe_global_wat!(
	"{}",
	interpolate "test",
	#[cfg()]
	#[cfg()]
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::unsafe_global_wat!("{}", par = "test");
//~^ ERROR: expected `const` or `interpolate`

js_bindgen::unsafe_global_wat!("{}", par = operator "test");
//~^ ERROR: expected `const` or `interpolate`

js_bindgen::unsafe_global_wat!("{}", interpolate "test", par = interpolate "test");
//~^ ERROR: named argument must come first

js_bindgen::unsafe_global_wat!("{par}", par = interpolate "test 1", par = interpolate "test 2");
//~^ ERROR: found duplicate named argument

js_bindgen::unsafe_global_wat!("{par}", #[cfg()] par = interpolate "test 1", #[cfg()] par = interpolate "test 2");
//~^ ERROR: found duplicate named argument

js_bindgen::unsafe_global_wat!("{}", #[cfg()] interpolate "test 1");
//~^ ERROR: `cfg` attributes are only supported on named arguments

js_bindgen::unsafe_global_wat!("{()}");
//~^ ERROR: template string named argument identifier

js_bindgen::unsafe_global_wat!("{par()}");
//~^ ERROR: template string named argument identifier

js_bindgen::unsafe_global_wat!("{[}");
//~^ ERROR: invalid template string named argument identifier: this file
// contains an unclosed delimiter

js_bindgen::unsafe_global_wat!("{par}");
//~^ ERROR: expected a named argument for `par`

js_bindgen::unsafe_global_wat!("", interpolate "test");
//~^ ERROR: expected no leftover arguments

js_bindgen::unsafe_global_wat!("", par = interpolate "test");
//~^ ERROR: expected no leftover arguments
