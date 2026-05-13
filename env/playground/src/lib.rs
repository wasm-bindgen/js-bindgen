/// ```rust
/// assert_eq!(playground::add(1, 1), 2);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
	left + right
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
