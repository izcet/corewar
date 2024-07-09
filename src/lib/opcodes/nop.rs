//mod opcodes;

pub fn nopf() -> u8 {
    return 0;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn validate_nop() {
		let expected = 0;
		let actual = nopf();
		assert_eq!(actual, expected)
	}
}
