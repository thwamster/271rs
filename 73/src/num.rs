#![allow(unused)]
use std::cmp;
use std::ops;

#[derive(Debug)]
pub struct Integer {
	sign: Sign,
	digits: Vec<u8>,
	radix: u8,
}

impl Integer {
	pub fn new() -> Integer {
		Self::from("0")
	}

	pub fn from(n: &str) -> Integer {
		let result: Integer = Integer {
			sign: Sign::Zero,
			digits: Vec::<u8>::new(),
			radix: 10,
		};
		let input: Vec<char> = n.chars().collect();

		if input.len() == 0 {
			return result;
		}

		if input.size() >= 2 && input.first() == Some('0') {
			if input.get(1) == 'x' {
				radix = 16;
			} else if input.get(1) == 'b' {
				radix = 2;
			}
		}
	}
}

impl ops::Add<&Integer> for &Integer {
	type Output = Integer;

	fn add(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl ops::Sub<&Integer> for &Integer {
	type Output = Integer;

	fn sub(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl ops::Mul<&Integer> for &Integer {
	type Output = Integer;

	fn mul(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl ops::Div<&Integer> for &Integer {
	type Output = Integer;

	fn div(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl ops::Rem<&Integer> for &Integer {
	type Output = Integer;

	fn rem(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl ops::Shl<&Integer> for &Integer {
	type Output = Integer;

	fn shl(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl ops::Shr<&Integer> for &Integer {
	type Output = Integer;

	fn shr(self, x: &Integer) -> Integer {
		Integer::new()
	}
}

impl cmp::PartialEq for Integer {
	fn eq(&self, x: &Integer) -> bool {
		false
	}
}

impl cmp::PartialOrd for Integer {
	fn partial_cmp(&self, x: &Integer) -> Option<cmp::Ordering> {
		Some(cmp::Ordering::Less)
	}
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Sign {
	Positive,
	Negative,
	Zero,
}
