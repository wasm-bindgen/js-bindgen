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
const IMPORT_EMBEDS: &[JsEmbed] = &[JsEmbed::new("js_sys", "identity")];
const IMPORTS: &[WireImport] = &[
	WireImport::closure_factory(
		"js_sys",
		"number.identity",
		&[WireImportInput::new("arg0", 0)],
		Some(WireImportOutput::new(0)),
		Some(WireImportBinding::new(
			Some("globalThis.identity"),
			"globalThis.identity(arg0)",
			IMPORT_EMBEDS,
		)),
		false,
	),
	WireImport::new(
		"js_sys",
		"wide.suspending",
		&[],
		Some(WireImportOutput::new(1)),
		Some(WireImportBinding::new(None, "globalThis.wide()", &[])),
		true,
	),
];
const IMPORT_WIRE: Wire = Wire::imports_with(PointerWidth::Wasm32, &IMPORT_TYPE_TABLE, IMPORTS);
const IMPORT_LEN: usize = wire_blob_len(&IMPORT_WIRE);
const IMPORT_RECORD: WireRecord<IMPORT_LEN> = WireRecord::new(&IMPORT_WIRE);

const EXPORT_I32_INPUT: WireExportInputType =
	WireExportInputType::new([I32, None, None, None], None);
const EXPORT_PTR_INPUT: WireExportInputType =
	WireExportInputType::new([I64, None, None, None], None);
const EXPORT_DIRECT_OUTPUT: WireExportOutputType = WireExportOutputType::new(
	ReturnMode::Direct,
	ReturnConv::Value(None),
	[I32, None, None, None],
	0,
	[0; 4],
	None,
);
const EXPORT_RESULT_UNIT_OUTPUT: WireExportOutputType = WireExportOutputType::new(
	ReturnMode::Indirect,
	ReturnConv::Result(None),
	[None, None, I32, I32],
	16,
	[0, 0, 0, 8],
	Some(ResultLayout::new(0, 1)),
);
const EXPORTS: &[WireExport] = &[
	WireExport::new_symbol(
		"exports",
		"foo",
		"foo.raw",
		&[WireExportInput::new("arg0", &EXPORT_I32_INPUT)],
		Some(WireExportOutput::new(&EXPORT_DIRECT_OUTPUT)),
	),
	WireExport::new_closure(
		"exports",
		"closure",
		0x1_0000_000c,
		&[WireExportInput::new("data", &EXPORT_PTR_INPUT)],
		Some(WireExportOutput::new(&EXPORT_RESULT_UNIT_OUTPUT)),
	),
];
const EXPORT_WIRE: Wire = Wire::exports_with(PointerWidth::Wasm64, EXPORTS);
const EXPORT_LEN: usize = wire_blob_len(&EXPORT_WIRE);
const EXPORT_RECORD: WireRecord<EXPORT_LEN> = WireRecord::new(&EXPORT_WIRE);

#[test]
fn imports() {
	let Record::Imports(group) = decode(IMPORT_RECORD.as_bytes()).unwrap() else {
		panic!("expected imports");
	};
	let imports = &group.imports;
	assert_eq!(imports.len(), 2);
	assert_eq!(imports[0].module, "js_sys");
	assert_eq!(imports[0].name, "number.identity");
	assert_eq!(imports[0].inputs[0].name, "arg0");
	assert_eq!(imports[0].inputs[0].js_conversion, Some("$slot1"));
	let binding = imports[0].binding.as_ref().unwrap();
	assert_eq!(binding.direct_expression, Some("globalThis.identity"));
	assert_eq!(binding.call_expression, "globalThis.identity(arg0)");
	assert_eq!(
		binding.embeds,
		[
			Embed {
				module: "js_sys",
				name: "input.convert",
			},
			Embed {
				module: "js_sys",
				name: "identity",
			},
		]
	);
	let slot = &imports[0].inputs[0].slots[0];
	assert_eq!(slot.rust, WatType::I32);
	assert_eq!(slot.js(), WatType::ExternRef);
	let conversion = slot.wat.as_ref().unwrap();
	assert_eq!(conversion.js, WatType::ExternRef);
	assert_eq!(conversion.imports.len(), 4);
	assert_eq!(conversion.imports[0].identifier, "support.function");
	assert!(matches!(
		&conversion.imports[0].kind,
		WatImportKind::Function {
			parameters,
			results,
		} if parameters == &[WatType::I32] && results == &[WatType::ExternRef]
	));
	assert!(matches!(
		&conversion.imports[1].kind,
		WatImportKind::Table {
			index_type: WatIndexType::I32,
			minimum: 2,
			maximum: None,
			element: RefType::ExternRef,
		}
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
	assert_eq!(imports[0].shim_kind, ImportShimKind::ClosureFactory);
	assert_eq!(imports[1].shim_kind, ImportShimKind::Normal);
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
	assert_eq!(binding.direct_expression, None);
	assert_eq!(binding.call_expression, "globalThis.wide()");
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
	assert_eq!(retptr.slot.rust, WatType::I32);
	assert_eq!(retptr.slot.js(), WatType::I32);
	assert_eq!(retptr.js_conversion, Some("$slot1 >>> 0"));
}

#[test]
fn javascript_catch_roundtrip() {
	const TABLE: WireImportTypeTable = WireImportTypeTable::new(
		&IMPORT_RETPTR_TYPE,
		IMPORT_INPUT_TYPES,
		IMPORT_OUTPUT_TYPES,
		JS_CATCH,
	);
	const IMPORTS: &[WireImport] = &[WireImport::new(
		"support",
		"fallible",
		&[],
		Some(WireImportOutput::new(1)),
		Some(WireImportBinding::new(None, "fallible()", &[])),
		false,
	)];
	const WIRE: Wire = Wire::imports_with(PointerWidth::Wasm32, &TABLE, IMPORTS);
	const LEN: usize = wire_blob_len(&WIRE);
	const RECORD: WireRecord<LEN> = WireRecord::new(&WIRE);

	let Record::Imports(group) = decode(RECORD.as_bytes()).unwrap() else {
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
}

#[test]
fn catch_payload_is_omitted_without_result_types() {
	const OUTPUT_TYPES: &[&WireImportOutputType] = &[&IMPORT_OUTPUT_TYPE_DIRECT];
	const TABLE: WireImportTypeTable =
		WireImportTypeTable::new(&IMPORT_RETPTR_TYPE, &[], OUTPUT_TYPES, JS_CATCH);
	const WIRE: Wire = Wire::imports_with(PointerWidth::Wasm32, &TABLE, &[]);
	const LEN: usize = wire_blob_len(&WIRE);
	const RECORD: WireRecord<LEN> = WireRecord::new(&WIRE);

	let Record::Imports(group) = decode(RECORD.as_bytes()).unwrap() else {
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
	assert_eq!(exports[0].module, "exports");
	assert_eq!(exports[0].name, "foo");
	assert_eq!(exports[0].pointer_width, PointerWidth::Wasm64);
	assert_eq!(exports[0].callee, Callee::Symbol { name: "foo.raw" });
	assert_eq!(exports[0].inputs[0].name, "arg0");
	assert_eq!(exports[0].inputs[0].kind, ExportInputKind::Value);
	assert_eq!(exports[0].inputs[0].slots[0].rust, WatType::I32);
	assert_eq!(exports[0].inputs[0].slots[0].js(), WatType::I32);
	let Some(ExportOutput::Direct { slot, .. }) = &exports[0].output else {
		panic!("expected direct output");
	};
	assert_eq!(slot.rust, WatType::I32);
	assert_eq!(slot.js(), WatType::I32);
	assert!(matches!(
		exports[1].callee,
		Callee::Closure {
			call_shim_offset: 0x1_0000_000c
		}
	));
	let Some(ExportOutput::Indirect { frame, result, .. }) = &exports[1].output else {
		panic!("expected indirect output");
	};
	assert_eq!(frame.slots.len(), 2);
	assert_eq!(
		*result,
		Some(ResultLayout {
			discriminant: 0,
			error: 1
		})
	);
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
