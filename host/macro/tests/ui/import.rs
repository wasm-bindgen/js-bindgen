js_bindgen::import_js!();
//~^ ERROR: expected `module = ...`

js_bindgen::import_js!(42);
//~^ ERROR: expected `module = ...`

js_bindgen::import_js!(foo);
//~^ ERROR: expected `module`

js_bindgen::import_js!(module);
//~^ ERROR: expected `module = ...`

js_bindgen::import_js!(module$);
//~^ ERROR: expected `module = ...`

js_bindgen::import_js!(module =);
//~^ ERROR: expected `module = ...`

js_bindgen::import_js!(foo = "bar");
//~^ ERROR: expected `module`

js_bindgen::import_js!(module = "foo");
//~^ ERROR: expected `name = ...`

js_bindgen::import_js!(module = "foo" $);
//~^ ERROR: expected a `,` after an attribute

js_bindgen::import_js!(module = "foo",);
//~^ ERROR: expected `name = ...`

js_bindgen::import_js!(module = "foo", 42);
//~^ ERROR: expected `name = ...`

js_bindgen::import_js!(module = "foo", "bar");
//~^ ERROR: expected `name = ...`

js_bindgen::import_js!(module = "foo", bar = Baz);
//~^ ERROR: expected `name`

js_bindgen::import_js!(module = "foo", bar = "baz");
//~^ ERROR: expected `name`

js_bindgen::import_js!(module = "foo", name = "bar");
//~^ ERROR: requires at least a string template

js_bindgen::import_js!(module = "foo", name = "bar",);
//~^ ERROR: requires at least a string template

js_bindgen::import_js!(module = "foo", name = "bar", 42);
//~^ ERROR: requires at least a string template

js_bindgen::import_js!(module = "foo", name = "bar", baz = qux);
//~^ ERROR: expected `required_embeds`

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds);
//~^ ERROR: expected `required_embeds =`

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds$);
//~^ ERROR: expected `required_embeds =`

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = Qux);
//~^ ERROR: expected array of string pairs

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = "qux");
//~^ ERROR: expected array of string pairs

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux", "quux")]
);
//~^^^^^ ERROR: requires at least a string template

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux", "quux")],
);
//~^^^^^ ERROR: requires at least a string template

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux", "quux")],
	42
);
//~^^ ERROR: requires at least a string template

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux", "quux")],
	baz = Qux
);
//~^^ ERROR: requires at least a string template

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [$("qux", "quux")],
	""
);
//~^^^ ERROR: expected string value

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [#[test]],
	"",
);
//~^^^ ERROR: expected `cfg`

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [#[cfg()]],
	"",
);
//~^^^ ERROR: leftover `cfg` attribute

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [#[cfg()] $],
	"",
);
//~^^^ ERROR: expected string value

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [#[cfg()] #[test]],
	"",
);
//~^^^ ERROR: expected `cfg`

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [#[cfg()] #[cfg()]],
	"",
);
//~^^^ ERROR: multiple `cfg`s in a row not supported

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux", "quux")$],
	""
);
//~^^^ ERROR: expected a `,` after a tuple

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux", "quux")]$
);
//~^^ ERROR: expected a `,` after an attribute
