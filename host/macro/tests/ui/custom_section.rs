js_bindgen::unsafe_embed_asm!("{}", interpolate Foo);
//~^ ERROR: cannot find value `Foo` in this scope

struct Bar;

js_bindgen::unsafe_embed_asm!("{}", interpolate Bar);
//~^ E0308

js_bindgen::unsafe_embed_asm!("{}", interpolate 42);
//~^ E0308

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = [42], "");
//~^ E0308

js_bindgen::embed_js!(module = "foo", name = "bar", required_embeds = [42], "");
//~^ E0308

js_bindgen::import_js!(module = "foo", name = "bar", required_embeds = ["qux"], "");
//~^ E0308

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux")],
	""
);
//~^^^ E0308

js_bindgen::import_js!(
	module = "foo",
	name = "bar",
	required_embeds = [("qux",)],
	""
);
//~^^^ E0308
