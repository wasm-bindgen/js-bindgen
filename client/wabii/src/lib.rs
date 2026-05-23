#![no_std]

macro_rules! include_wat {
	($path:literal) => {
        #[expect(unused, reason = "link_section")]
		const _: () = {
			const WAT: &[u8] = include_bytes!($path);

			#[repr(C)]
			struct Layout<const N: usize>([::core::primitive::u8; 4], [::core::primitive::u8; N]);

			#[unsafe(link_section = "js_bindgen.wat")]
			static CUSTOM_SECTION: Layout<{ WAT.len() }> = Layout(
                #[expect(clippy::cast_possible_truncation, reason = "link_section")]
				::core::primitive::u32::to_le_bytes(WAT.len() as ::core::primitive::u32),
				*include_bytes!($path),
			);
		};
	};
}

pub mod random;
pub mod stdio;
pub mod time;
