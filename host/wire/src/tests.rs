use alloc::vec;
use alloc::vec::Vec;

use crate::abi::{
	FromJsConv, IntoJsConv, JsCatch as AbiJsCatch, JsEmbed, RefType, ReturnConv, ReturnMode, Sret,
	WatCatch as AbiWatCatch, WatConv as AbiWatConv, WatImport as AbiWatImport,
	WatImportKind as AbiWatImportKind, WatIndexType, WatLocal as AbiWatLocal, WatSlot, WatType,
};
use crate::model::*;
use crate::*;

const I32: Option<WatSlot> = Some(WatSlot::plain(WatType::I32));
const I64: Option<WatSlot> = Some(WatSlot::plain(WatType::I64));
const WAT_IMPORTS: &[AbiWatImport] = &[
	AbiWatImport::new(
		"env",
		"support.function",
		"support.function",
		None,
		AbiWatImportKind::Function {
			parameters: &[WatType::I32],
			results: &[WatType::ExternRef],
		},
	),
	AbiWatImport::new(
		"support",
		"table",
		"support.import.table",
		Some("support.table"),
		AbiWatImportKind::Table {
			index_type: WatIndexType::I32,
			minimum: 2,
			maximum: None,
			element: RefType::ExternRef,
		},
	),
	AbiWatImport::new(
		"support",
		"table64",
		"support.import.table64",
		None,
		AbiWatImportKind::Table {
			index_type: WatIndexType::I64,
			minimum: 3,
			maximum: Some(8),
			element: RefType::FuncRef,
		},
	),
	AbiWatImport::new(
		"support",
		"exception",
		"support.exception",
		Some("support.exception"),
		AbiWatImportKind::Tag {
			parameters: &[WatType::ExternRef],
		},
	),
];
const WAT_LOCALS: &[AbiWatLocal] = &[
	AbiWatLocal::new("support.value", WatType::ExternRef),
	AbiWatLocal::new("support.index", WatType::I32),
];
const WAT_CATCH: WireImportCatch = WireImportCatch::Wasm(AbiWatCatch::new(
	WAT_IMPORTS,
	WAT_LOCALS,
	"(try_table (catch $support.exception $support.catch)",
	") local.set $support.index",
));
const JS_CATCH_EMBEDS: &[JsEmbed] = &[JsEmbed::new("support", "table")];
const JS_CATCH: WireImportCatch = WireImportCatch::JavaScript(AbiJsCatch::new(
	JS_CATCH_EMBEDS,
	"} catch ($error) { return false } }",
	"} catch ($error) { store($error) } }",
));
const CONVERTED_I32: Option<WatSlot> = Some(WatSlot::new(
	WatType::I32,
	Some(AbiWatConv::new(
		WAT_IMPORTS,
		WAT_LOCALS,
		"call $support.function (@reloc)",
		WatType::ExternRef,
	)),
));

const IMPORT_INPUT_TYPE: WireImportInputType = WireImportInputType::new(
	[CONVERTED_I32, None, None, None],
	Some(IntoJsConv::new("$slot1").with_embed("js_sys", "input.convert")),
);
const IMPORT_INPUT_TYPES: &[&WireImportInputType] = &[&IMPORT_INPUT_TYPE];
const IMPORT_RETPTR_TYPE: WireImportInputType = WireImportInputType::new(
	[I32, None, None, None],
	Some(IntoJsConv::new("$slot1 >>> 0").with_embed("js_sys", "retptr.convert")),
);
const IMPORT_OUTPUT_TYPE_DIRECT: WireImportOutputType = WireImportOutputType::new(
	ReturnMode::Direct,
	ReturnConv::Value(None),
	None,
	[I32, None, None, None],
);
const IMPORT_OUTPUT_TYPE_INDIRECT: WireImportOutputType = WireImportOutputType::new(
	ReturnMode::Indirect,
	ReturnConv::Result(Some(
		FromJsConv::slot1("$ret[0]")
			.prepare("prepare")
			.slot2("$ret[1]")
			.with_embed("js_sys", "output.convert"),
	)),
	Some(Sret::Slots("store")),
	[I64, I64, None, None],
);
const IMPORT_OUTPUT_TYPES: &[&WireImportOutputType] =
	&[&IMPORT_OUTPUT_TYPE_DIRECT, &IMPORT_OUTPUT_TYPE_INDIRECT];
const IMPORT_TYPE_TABLE: WireImportTypeTable = WireImportTypeTable::new(
	&IMPORT_RETPTR_TYPE,
	IMPORT_INPUT_TYPES,
	IMPORT_OUTPUT_TYPES,
	WAT_CATCH,
);
const IDENTITY_PATH: WireGlobalPath = WireGlobalPath::new(None, None, "identity");
const WIDE_PATH: WireGlobalPath = WireGlobalPath::new(None, None, "wide");
const IMPORTS: &[WireImport] = &[
	WireImport::new(
		"js_sys",
		"number.identity",
		&[WireImportInput::new(0)],
		Some(WireImportOutput::new(0)),
		Some(WireImportBinding::CallGlobal {
			target: IDENTITY_PATH,
			variadic: false,
		}),
		false,
	),
	WireImport::new(
		"js_sys",
		"wide.suspending",
		&[],
		Some(WireImportOutput::new(1)),
		Some(WireImportBinding::CallGlobal {
			target: WIDE_PATH,
			variadic: false,
		}),
		true,
	),
];
const IMPORT_WIRE: Wire = Wire::imports_with(PointerWidth::Wasm32, &IMPORT_TYPE_TABLE, IMPORTS);
const IMPORT_LEN: usize = wire_blob_len(&IMPORT_WIRE);
const IMPORT_RECORD: WireRecord<IMPORT_LEN> = WireRecord::new(&IMPORT_WIRE);

const EXPORT_I32_INPUT: WireExportInputType =
	WireExportInputType::new([I32, None, None, None], None);
const EXPORT_DIRECT_OUTPUT: WireExportOutputType =
	WireExportOutputType::direct([I32, None, None, None], None);
const EXPORT_VOID_OUTPUT: WireExportOutputType =
	WireExportOutputType::direct([None, None, None, None], None);
const EXPORT_RESULT_UNIT_OUTPUT: WireExportOutputType = WireExportOutputType::indirect(
	[None, None, I32, I32],
	ReturnConv::Result(None),
	WireReturnFrame::new(16, [0, 0, 0, 8]),
	Some(ResultLayout::new(0, 1)),
);
const EXPORTS: &[WireExport] = &[
	WireExport::new_symbol(
		"exports",
		"foo",
		"foo.raw",
		&[WireExportInput::new(&EXPORT_I32_INPUT)],
		Some(WireExportOutput::new(&EXPORT_DIRECT_OUTPUT)),
	),
	WireExport::new_symbol(
		"exports",
		"unit",
		"unit.raw",
		&[],
		Some(WireExportOutput::new(&EXPORT_VOID_OUTPUT)),
	),
];
const EXPORT_WIRE: Wire = Wire::exports_with(PointerWidth::Wasm64, EXPORTS);
const EXPORT_LEN: usize = wire_blob_len(&EXPORT_WIRE);
const EXPORT_RECORD: WireRecord<EXPORT_LEN> = WireRecord::new(&EXPORT_WIRE);

const CLOSURE_FACTORY_INPUT_TYPE: WireImportInputType = WireImportInputType::new(
	[I64, None, None, None],
	Some(IntoJsConv::new("BigInt.asUintN(64, $slot1)")),
);
const CLOSURE_FACTORY_OUTPUT_TYPE: WireImportOutputType = WireImportOutputType::new(
	ReturnMode::Direct,
	ReturnConv::Value(None),
	None,
	[CONVERTED_I32, None, None, None],
);
const CLOSURE_FACTORY: WireClosureFactory = WireClosureFactory::new(
	"closures.example@1.0.0:module:10:4:mutable",
	JsEmbed::new("js_sys", "closure.make_mut"),
	&CLOSURE_FACTORY_INPUT_TYPE,
	&CLOSURE_FACTORY_OUTPUT_TYPE,
);
const CLOSURE_DISPATCHER_DATA_TYPE: WireExportInputType =
	WireExportInputType::new([I64, None, None, None], None);
const CLOSURE_INPUTS: &[WireExportInput] = &[
	WireExportInput::new(&CLOSURE_DISPATCHER_DATA_TYPE),
	WireExportInput::new(&EXPORT_I32_INPUT),
];
const CLOSURE: WireClosure = WireClosure::new(
	CLOSURE_FACTORY,
	8,
	CLOSURE_INPUTS,
	Some(WireExportOutput::new(&EXPORT_RESULT_UNIT_OUTPUT)),
);
const CLOSURE_WIRE: Wire = Wire::closure_with(PointerWidth::Wasm64, CLOSURE);
const CLOSURE_LEN: usize = wire_blob_len(&CLOSURE_WIRE);
const CLOSURE_RECORD: WireRecord<CLOSURE_LEN> = WireRecord::new(&CLOSURE_WIRE);

#[test]
fn imports() {
	let Record::Imports(group) = decode(IMPORT_RECORD.as_bytes()).unwrap() else {
		panic!("expected imports");
	};
	let imports = &group.imports;
	assert_eq!(imports.len(), 2);
	assert_eq!(
		(imports[0].module, imports[0].name),
		("js_sys", "number.identity")
	);
	assert_eq!(imports[0].inputs[0].js_conversion, Some("$slot1"));
	let binding = imports[0].binding.as_ref().unwrap();
	assert_eq!(
		binding.embeds,
		[Embed {
			module: "js_sys",
			name: "input.convert",
		}]
	);
	let slot = &imports[0].inputs[0].slots[0];
	assert_eq!((slot.rust, slot.js()), (WatType::I32, WatType::ExternRef));
	let conversion = slot.wat.as_ref().unwrap();
	assert!(matches!(
		&conversion.imports[0].kind,
		WatImportKind::Function {
			parameters,
			results,
		} if parameters == &[WatType::I32] && results == &[WatType::ExternRef]
	));
	assert!(matches!(
		&conversion.imports[2].kind,
		WatImportKind::Table {
			index_type: WatIndexType::I64,
			minimum: 3,
			maximum: Some(8),
			element: RefType::FuncRef,
		}
	));
	assert!(matches!(
		&conversion.imports[3].kind,
		WatImportKind::Tag { parameters } if parameters == &[WatType::ExternRef]
	));
	assert_eq!(
		conversion.locals.as_ref(),
		&[
			WatLocal {
				name: "support.value",
				ty: WatType::ExternRef,
			},
			WatLocal {
				name: "support.index",
				ty: WatType::I32,
			},
		]
	);
	assert!(imports[1].suspending);
	let Some(ImportCatch::Wasm(catch)) = &group.catch else {
		panic!("expected Wasm catch metadata");
	};
	assert_eq!(catch.imports.len(), 4);
	assert_eq!(catch.locals.len(), 2);
	assert_eq!(
		catch.try_,
		"(try_table (catch $support.exception $support.catch)"
	);
	assert_eq!(catch.catch, ") local.set $support.index");
	let binding = imports[1].binding.as_ref().unwrap();
	assert_eq!(
		binding.embeds,
		[
			Embed {
				module: "js_sys",
				name: "retptr.convert",
			},
			Embed {
				module: "js_sys",
				name: "output.convert",
			},
		]
	);
	let ImportOutputAbi::Indirect { retptr, .. } = &imports[1].output.as_ref().unwrap().abi else {
		panic!("expected indirect output");
	};
	assert_eq!(retptr.js_conversion, Some("$slot1 >>> 0"));
}

#[test]
fn import_bindings() {
	const fn import(name: &'static str, binding: WireImportBinding) -> WireImport {
		WireImport::new("test", name, &[], None, Some(binding), false)
	}

	const TARGET: WireGlobalPath = WireGlobalPath::new(Some("namespace"), Some("Object"), "member");
	const EMBED: JsEmbed = JsEmbed::new("support", "call");
	const TABLE: WireImportTypeTable =
		WireImportTypeTable::new(&IMPORT_RETPTR_TYPE, &[], &[], JS_CATCH);
	const IMPORTS: &[WireImport] = &[
		import(
			"call_global",
			WireImportBinding::CallGlobal {
				target: TARGET,
				variadic: true,
			},
		),
		import(
			"call_method",
			WireImportBinding::CallMethod {
				name: "method",
				variadic: false,
			},
		),
		import("index_get", WireImportBinding::IndexGet),
		import("call_embed", WireImportBinding::CallEmbed(EMBED)),
	];
	const WIRE: Wire = Wire::imports_with(PointerWidth::Wasm32, &TABLE, IMPORTS);
	const LEN: usize = wire_blob_len(&WIRE);
	const RECORD: WireRecord<LEN> = WireRecord::new(&WIRE);

	let Record::Imports(group) = decode(RECORD.as_bytes()).unwrap() else {
		panic!("expected imports");
	};
	let kinds: Vec<_> = group
		.imports
		.iter()
		.map(|import| import.binding.as_ref().unwrap().kind)
		.collect();
	let target = GlobalPath {
		namespace: Some("namespace"),
		object: Some("Object"),
		name: "member",
	};
	assert_eq!(
		kinds,
		[
			ImportBindingKind::CallGlobal {
				target,
				variadic: true,
			},
			ImportBindingKind::CallMethod {
				name: "method",
				variadic: false,
			},
			ImportBindingKind::IndexGet,
			ImportBindingKind::CallEmbed(Embed {
				module: "support",
				name: "call",
			}),
		]
	);
	let embed_binding = group.imports.last().unwrap().binding.as_ref().unwrap();
	assert_eq!(
		embed_binding.embeds,
		[Embed {
			module: "support",
			name: "call",
		}]
	);
}

#[test]
fn javascript_catch() {
	const RESULT_TABLE: WireImportTypeTable = WireImportTypeTable::new(
		&IMPORT_RETPTR_TYPE,
		IMPORT_INPUT_TYPES,
		IMPORT_OUTPUT_TYPES,
		JS_CATCH,
	);
	const RESULT_IMPORTS: &[WireImport] = &[WireImport::new(
		"support",
		"fallible",
		&[],
		Some(WireImportOutput::new(1)),
		Some(WireImportBinding::CallGlobal {
			target: WireGlobalPath::new(None, None, "fallible"),
			variadic: false,
		}),
		false,
	)];
	const RESULT_WIRE: Wire =
		Wire::imports_with(PointerWidth::Wasm32, &RESULT_TABLE, RESULT_IMPORTS);
	const RESULT_LEN: usize = wire_blob_len(&RESULT_WIRE);
	const RESULT_RECORD: WireRecord<RESULT_LEN> = WireRecord::new(&RESULT_WIRE);
	const PLAIN_OUTPUTS: &[&WireImportOutputType] = &[&IMPORT_OUTPUT_TYPE_DIRECT];
	const PLAIN_TABLE: WireImportTypeTable =
		WireImportTypeTable::new(&IMPORT_RETPTR_TYPE, &[], PLAIN_OUTPUTS, JS_CATCH);
	const PLAIN_WIRE: Wire = Wire::imports_with(PointerWidth::Wasm32, &PLAIN_TABLE, &[]);
	const PLAIN_LEN: usize = wire_blob_len(&PLAIN_WIRE);
	const PLAIN_RECORD: WireRecord<PLAIN_LEN> = WireRecord::new(&PLAIN_WIRE);

	let Record::Imports(group) = decode(RESULT_RECORD.as_bytes()).unwrap() else {
		panic!("expected imports");
	};
	let Some(ImportCatch::JavaScript(catch)) = group.catch else {
		panic!("expected JavaScript catch metadata");
	};
	assert_eq!(catch.direct, "} catch ($error) { return false } }");
	assert_eq!(catch.indirect, "} catch ($error) { store($error) } }");
	assert_eq!(
		catch.embeds.as_ref(),
		&[Embed {
			module: "support",
			name: "table"
		}]
	);
	assert_eq!(
		group.imports[0].output.as_ref().unwrap().error,
		ImportErrorMode::CatchInJavaScript
	);

	let Record::Imports(group) = decode(PLAIN_RECORD.as_bytes()).unwrap() else {
		panic!("expected imports");
	};
	assert!(group.catch.is_none());
}

#[test]
fn exports() {
	let Record::Exports(exports) = decode(EXPORT_RECORD.as_bytes()).unwrap() else {
		panic!("expected exports");
	};
	assert_eq!(exports.len(), 2);
	assert_eq!((exports[0].module, exports[0].name), ("exports", "foo"));
	assert_eq!(exports[0].pointer_width, PointerWidth::Wasm64);
	assert_eq!(exports[0].callee, Callee::Symbol { name: "foo.raw" });
	assert_eq!(exports[0].inputs[0].kind, ExportInputKind::Value);
	assert_eq!(exports[0].inputs[0].slots[0].rust, WatType::I32);
	let Some(ExportOutput::Direct { slot, .. }) = &exports[0].output else {
		panic!("expected direct output");
	};
	let slot = slot.as_ref().expect("expected one direct output slot");
	assert_eq!(slot.rust, WatType::I32);
	assert_eq!((exports[1].module, exports[1].name), ("exports", "unit"));
	assert!(
		exports[1]
			.output
			.as_ref()
			.is_some_and(ExportOutput::is_void)
	);
}

#[test]
fn closure() {
	let Record::Closure(closure) = decode(CLOSURE_RECORD.as_bytes()).unwrap() else {
		panic!("expected a closure");
	};
	assert_eq!(
		closure.factory.raw_symbol,
		"closures.example@1.0.0:module:10:4:mutable"
	);
	assert_eq!(
		closure.factory.helper,
		Embed {
			module: "js_sys",
			name: "closure.make_mut",
		}
	);
	assert_eq!(
		closure.factory.input.js_conversion,
		Some("BigInt.asUintN(64, $slot1)")
	);
	assert!(closure.factory.embeds.is_empty());
	let ImportOutputAbi::Direct { slot, conversion } = &closure.factory.output.abi else {
		panic!("expected a direct factory output");
	};
	assert!(conversion.is_none());
	assert_eq!((slot.rust, slot.js()), (WatType::I32, WatType::ExternRef));

	assert_eq!(closure.pointer_width, PointerWidth::Wasm64);
	assert_eq!(
		closure
			.inputs
			.iter()
			.map(|input| input.kind)
			.collect::<Vec<_>>(),
		[ExportInputKind::ClosureData, ExportInputKind::Value]
	);
	assert_eq!(closure.call_shim_offset, 8);
	let Some(ExportOutput::Indirect { frame, result, .. }) = &closure.output else {
		panic!("expected an indirect dispatcher output");
	};
	assert_eq!(frame.size, 16);
	assert_eq!(
		frame
			.slots
			.iter()
			.map(|slot| slot.offset)
			.collect::<Vec<_>>(),
		[0, 8]
	);
	assert_eq!(*result, Some(ResultLayout::new(0, 1)));
}

#[test]
fn closure_identity() {
	const OTHER_ORIGIN: WireClosureFactory = WireClosureFactory::new(
		"another.call.site",
		JsEmbed::new("js_sys", "closure.make_mut"),
		&CLOSURE_FACTORY_INPUT_TYPE,
		&CLOSURE_FACTORY_OUTPUT_TYPE,
	);
	const OTHER_ORIGIN_CLOSURE: WireClosure = WireClosure::new(
		OTHER_ORIGIN,
		8,
		CLOSURE_INPUTS,
		Some(WireExportOutput::new(&EXPORT_RESULT_UNIT_OUTPUT)),
	);
	const OTHER_ORIGIN_WIRE: Wire = Wire::closure_with(PointerWidth::Wasm64, OTHER_ORIGIN_CLOSURE);
	const OTHER_ORIGIN_LEN: usize = wire_blob_len(&OTHER_ORIGIN_WIRE);
	const OTHER_ORIGIN_RECORD: WireRecord<OTHER_ORIGIN_LEN> = WireRecord::new(&OTHER_ORIGIN_WIRE);

	const OTHER_HELPER: WireClosureFactory = WireClosureFactory::new(
		"another.call.site",
		JsEmbed::new("js_sys", "closure.make_once"),
		&CLOSURE_FACTORY_INPUT_TYPE,
		&CLOSURE_FACTORY_OUTPUT_TYPE,
	);
	const OTHER_HELPER_CLOSURE: WireClosure = WireClosure::new(
		OTHER_HELPER,
		8,
		CLOSURE_INPUTS,
		Some(WireExportOutput::new(&EXPORT_RESULT_UNIT_OUTPUT)),
	);
	const OTHER_HELPER_WIRE: Wire = Wire::closure_with(PointerWidth::Wasm64, OTHER_HELPER_CLOSURE);
	const OTHER_HELPER_LEN: usize = wire_blob_len(&OTHER_HELPER_WIRE);
	const OTHER_HELPER_RECORD: WireRecord<OTHER_HELPER_LEN> = WireRecord::new(&OTHER_HELPER_WIRE);

	const OTHER_CALL: WireClosure = WireClosure::new(
		CLOSURE_FACTORY,
		16,
		CLOSURE_INPUTS,
		Some(WireExportOutput::new(&EXPORT_RESULT_UNIT_OUTPUT)),
	);
	const OTHER_CALL_WIRE: Wire = Wire::closure_with(PointerWidth::Wasm64, OTHER_CALL);
	const OTHER_CALL_LEN: usize = wire_blob_len(&OTHER_CALL_WIRE);
	const OTHER_CALL_RECORD: WireRecord<OTHER_CALL_LEN> = WireRecord::new(&OTHER_CALL_WIRE);

	let Record::Closure(original) = decode(CLOSURE_RECORD.as_bytes()).unwrap() else {
		panic!("expected a closure");
	};
	let Record::Closure(other_origin) = decode(OTHER_ORIGIN_RECORD.as_bytes()).unwrap() else {
		panic!("expected a closure");
	};
	let Record::Closure(other_helper) = decode(OTHER_HELPER_RECORD.as_bytes()).unwrap() else {
		panic!("expected a closure");
	};
	let Record::Closure(other_call) = decode(OTHER_CALL_RECORD.as_bytes()).unwrap() else {
		panic!("expected a closure");
	};

	assert_eq!(original.call_identity, other_origin.call_identity);
	assert_ne!(original.factory.raw_symbol, other_origin.factory.raw_symbol);
	assert_ne!(original.factory.helper, other_helper.factory.helper);
	assert_eq!(original.call_identity, other_helper.call_identity);
	assert_ne!(original.call_identity, other_call.call_identity);
}

#[test]
fn invalid_records() {
	let mut version = IMPORT_RECORD.as_bytes().to_vec();
	version[8] = 0xff;
	assert!(matches!(
		decode(&version).unwrap_err().kind(),
		ErrorKind::UnsupportedVersion(_)
	));

	let mut kind = IMPORT_RECORD.as_bytes().to_vec();
	kind[11] = 0xff;
	assert_eq!(
		decode(&kind).unwrap_err().kind(),
		&ErrorKind::UnknownRecordKind(0xff)
	);

	// Eight bytes is a valid pointer width, but it conflicts with this record's
	// 32-bit return pointer.
	let mut pointer_width = IMPORT_RECORD.as_bytes().to_vec();
	pointer_width[10] = 8;
	assert!(matches!(
		decode(&pointer_width).unwrap_err().kind(),
		ErrorKind::InvalidValue(_)
	));

	let mut trailing = Vec::from(IMPORT_RECORD.as_bytes().as_slice());
	trailing.extend(vec![0]);
	assert_eq!(
		decode(&trailing).unwrap_err().kind(),
		&ErrorKind::TrailingBytes(1)
	);
}

#[test]
fn record_stream() {
	let first = IMPORT_RECORD.as_bytes();
	let second = EXPORT_RECORD.as_bytes();
	let mut section = Vec::new();
	section.extend_from_slice(&u32::try_from(first.len()).unwrap().to_le_bytes());
	section.extend_from_slice(first);
	section.extend_from_slice(&u32::try_from(second.len()).unwrap().to_le_bytes());
	section.extend_from_slice(second);

	let records = WireRecords::new(&section)
		.collect::<Result<Vec<_>, _>>()
		.unwrap();
	assert_eq!(records, [first.as_slice(), second.as_slice()]);

	let header_error = WireRecords::new(&[0, 0, 0]).next().unwrap().unwrap_err();
	assert_eq!(header_error.offset(), 0);
	assert_eq!(header_error.kind(), &ErrorKind::UnexpectedEnd { needed: 4 });

	let truncated = [5, 0, 0, 0, 1, 2];
	let payload_error = WireRecords::new(&truncated).next().unwrap().unwrap_err();
	assert_eq!(payload_error.offset(), 4);
	assert_eq!(
		payload_error.kind(),
		&ErrorKind::UnexpectedEnd { needed: 5 }
	);
}
