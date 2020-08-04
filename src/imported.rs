pub fn square(num: i64) -> i64 {
	num * num
}

pub fn fixed_square(num: i64) -> i64 {
	num * num
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
	fn test_fixed_square() {
		assert_eq!(fixed_square(2), 4);
		assert_eq!(fixed_square(5), 25);
	}
}
