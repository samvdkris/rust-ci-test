pub fn square(num: i64) -> i64 {
	num * num
}

pub fn bad_square(num: i64) -> i64 {
	num * 2
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_square() {
		assert_eq!(square(2), 4);
		assert_eq!(square(5), 25);
	}

	#[test]
	fn test_bad_square() {
		assert_eq!(bad_square(2), 4);
		assert_eq!(bad_square(5), 25);
	}
}
