js_bindgen::import_js!();
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(42);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(foo);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module+);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module =);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = 42);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = Foo);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(foo = "bar");
//~^ ERROR: expected `module`

js_bindgen::import_js!(module = "foo");
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = "foo",);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = "foo", 42);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = "foo", "bar");
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = "foo", bar = Baz);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(module = "foo", bar = "baz");
//~^ ERROR: expected `name`

js_bindgen::import_js!(module = "foo", name = "bar");
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(module = "foo", name = "bar",);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(module = "foo", name = "bar", 42);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(module = "foo", name = "bar", baz = Qux);
//~^ ERROR: expected array of string pairs

js_bindgen::import_js!(module = "foo", name = "bar", baz = "qux");
//~^ ERROR: expected array of string pairs

js_bindgen::import_js!(module = "foo", name = "bar", baz = ["qux"]);
//~^ ERROR: expected `required_embeds`

js_bindgen::import_js!(module = "foo", name = "bar", baz = [("qux")]);
//~^ ERROR: expected a `,` after a string value

js_bindgen::import_js!(module = "foo", name = "bar", baz = [("qux",)]);
//~^ ERROR: expected string value

js_bindgen::import_js!(module = "foo", name = "bar", baz = [("qux", "quux")]);
//~^ ERROR: expected `required_embeds`

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = [("qux", "quux")]);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = [("qux", "quux")],);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = [("qux", "quux")], 42);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = [("qux", "quux")], baz = Qux);
//~^ ERROR: requires at least a string argument
