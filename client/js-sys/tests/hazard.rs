use js_bindgen_test::test;
use js_sys::hazard::{
	EmptySlot, FromJS, FromJsConv, IntoJS, IntoJsConv, Slot, WasmAbi, WatConv, WatType,
};
use js_sys::{Closure, closure, js_sys};

js_bindgen::embed_js!(
	module = "hazard",
	name = "pair",
	"(value) => value[0] === 1 && value[1] === 2",
);
js_bindgen::embed_js!(
	module = "hazard",
	name = "quad",
	"(value) => value.length === 4 &&",
	"value[0] === 1 && value[1] === 2 && value[2] === 3 && value[3] === 4",
);
js_bindgen::embed_js!(
	module = "hazard",
	name = "invoke_quad",
	"(callback) => callback([1, 2, 3, 4])",
);
js_bindgen::embed_js!(module = "hazard", name = "identity", "value => value");

#[js_sys]
fn arg0(arg0: i32) -> i32 {
	arg0
}

#[repr(transparent)]
struct NumberSlot(u32);

// SAFETY: `NumberSlot` is an i32 carrier converted to a JS Number on input.
unsafe impl Slot for NumberSlot {
	const WAT_TYPE: Option<WatType> = Some(WatType::I32);
	const INTO_JS_WAT_CONV: Option<WatConv> =
		Some(WatConv::new(&[], &[], "f64.convert_i32_u", WatType::F64));
	const FROM_JS_WAT_CONV: Option<WatConv> =
		Some(WatConv::new(&[], &[], "i32.trunc_sat_f64_u", WatType::F64));
}

struct Pair(u32, u32);

struct Quad(u32, u32, u32, u32);

// SAFETY: `Pair` is represented by its two `u32` fields in order.
unsafe impl WasmAbi for Pair {
	type Slot1 = NumberSlot;
	type Slot2 = NumberSlot;
	type Slot3 = EmptySlot;
	type Slot4 = EmptySlot;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(
			NumberSlot(self.0),
			NumberSlot(self.1),
			EmptySlot::new(),
			EmptySlot::new(),
		)
	}

	fn join(slot1: Self::Slot1, slot2: Self::Slot2, _: Self::Slot3, _: Self::Slot4) -> Self {
		Self(slot1.0, slot2.0)
	}
}

// SAFETY: `Pair` lowers to two reusable `NumberSlot` carriers before the JS
// conversion combines them into one logical argument.
unsafe impl IntoJS for Pair {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("[$slot1, $slot2]"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: `Quad` is represented by its four `u32` fields in order.
unsafe impl WasmAbi for Quad {
	type Slot1 = NumberSlot;
	type Slot2 = NumberSlot;
	type Slot3 = NumberSlot;
	type Slot4 = NumberSlot;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(
			NumberSlot(self.0),
			NumberSlot(self.1),
			NumberSlot(self.2),
			NumberSlot(self.3),
		)
	}

	fn join(
		slot1: Self::Slot1,
		slot2: Self::Slot2,
		slot3: Self::Slot3,
		slot4: Self::Slot4,
	) -> Self {
		Self(slot1.0, slot2.0, slot3.0, slot4.0)
	}
}

// SAFETY: `Quad` lowers to four reusable `NumberSlot` carriers before the JS
// conversion combines them into one logical argument.
unsafe impl IntoJS for Quad {
	const JS_CONV: Option<IntoJsConv> = Some(IntoJsConv::new("[$slot1, $slot2, $slot3, $slot4]"));

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

// SAFETY: The JavaScript conversion splits a four-element numeric array into
// the four `NumberSlot` carriers expected by `Quad`.
unsafe impl FromJS for Quad {
	const JS_CONV: Option<FromJsConv> = Some(
		FromJsConv::slot1("$value[0]")
			.slot2("$value[1]")
			.slot3("$value[2]")
			.slot4("$value[3]"),
	);

	type Abi = Self;

	fn from_abi(raw: Self::Abi) -> Self {
		raw
	}
}

#[test]
fn input_slot_conversions() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "pair")]
		fn pair(value: Pair) -> bool;

		#[js_sys(js_embed = "quad")]
		fn quad(value: Quad) -> bool;
	}

	assert!(pair(Pair(1, 2)));
	assert!(quad(Quad(1, 2, 3, 4)));
}

#[test]
fn from_js_slot_conversions() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "invoke_quad")]
		fn invoke_quad(callback: &Closure<dyn FnMut(Quad) -> bool>) -> bool;
	}

	let callback = closure!(dyn FnMut(Quad) -> bool, |Quad(a, b, c, d)| {
		(a, b, c, d) == (1, 2, 3, 4)
	});
	assert!(invoke_quad(&callback));
}
