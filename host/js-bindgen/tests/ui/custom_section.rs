js_bindgen::unsafe_embed_asm!("{}", interpolate Foo);
//~^ ERROR: cannot find value `Foo` in this scope

struct Bar;

js_bindgen::unsafe_embed_asm!("{}", interpolate Bar);
//~^ ERROR: mismatched types

js_bindgen::unsafe_embed_asm!("{}", interpolate 42);
//~^ ERROR: mismatched types
