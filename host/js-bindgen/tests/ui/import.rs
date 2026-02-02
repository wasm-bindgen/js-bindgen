js_bindgen::import_js!();
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(42);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(foo);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(name);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(name+);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(name =);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(name = 42);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(name = Foo);
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::import_js!(foo = "bar");
//~^ ERROR: expected `name`

js_bindgen::import_js!(name = "foo");
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(name = "foo",);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(name = "foo", 42);
//~^ ERROR: requires at least a string argument

js_bindgen::import_js!(name = "foo", bar = Baz);
//~^ ERROR: expected `required_embed` or `no_import`

js_bindgen::import_js!(name = "foo", bar = "baz");
//~^ ERROR: expected `required_embed` or `no_import`

js_bindgen::embed_js!();
//~^ ERROR: expected `<attribute> = "..."`

js_bindgen::embed_js!(name = "foo");
//~^ ERROR: requires at least a string argument
