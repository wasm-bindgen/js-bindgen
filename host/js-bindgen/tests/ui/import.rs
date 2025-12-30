js_bindgen::js_import!();
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(42);
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(foo);
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(name);
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(name+);
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(name =);
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(name = 42);
//~^ ERROR: expected `name = "..."`

js_bindgen::js_import!(name = "foo");
//~^ ERROR: expected `name = "...",` and a list of string literals

js_bindgen::js_import!(name = "foo",);
//~^ ERROR: requires at least a string argument

js_bindgen::js_import!(name = "foo", 55);
//~^ ERROR: requires at least a string argument
