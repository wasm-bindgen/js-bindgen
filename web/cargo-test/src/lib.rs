pub fn add(left: u64, right: u64) -> u64 {
	left + right
}

#[cfg(test)]
mod tests {
	wcov::ensure_linked!();

	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}

	#[test]
	#[should_panic]
	fn test_should_panic() {
		panic!()
	}

	#[test]
	#[ignore = "test2"]
	fn test_ignore() {
		panic!()
	}
}
