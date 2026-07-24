#![feature(random)]
use std::random::random;
use std::time::Instant;

fn main() {
	let ins = Instant::now();

	let bits: u128 = random(..);
	let g1 = (bits >> 96) as u32;
	let g2 = (bits >> 80) as u16;
	let g3 = (0x4000 | (bits >> 64) & 0x0fff) as u16;
	let g4 = (0x8000 | (bits >> 48) & 0x3fff) as u16;
	let g5 = (bits & 0xffffffffffff) as u64;
	let uuid = format!("{g1:08x}-{g2:04x}-{g3:04x}-{g4:04x}-{g5:012x}");

	let elapsed = ins.elapsed();
	println!("result: {uuid}, cost: {elapsed:?}");
}

#[cfg(test)]
mod tests {
	#[test]
	#[should_panic]
	fn test1() {
		panic!()
	}

	#[test]
	#[ignore = "test2"]
	fn test2() {
		panic!()
	}

	#[test]
	fn test3() {}
}
