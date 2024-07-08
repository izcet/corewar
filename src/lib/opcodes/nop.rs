pub fn nop() -> None {
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn validate_nop() {
		let expected = 0;
		let actual = 0;
		assert_eq!(actual, expected)
	}
}
