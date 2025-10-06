#![allow(unused)]
use std::cmp;
use std::ops;

#[derive(Debug)]
pub struct f16 {
	bits: Vec<u8>,
}

impl f16 {
	pub fn new() -> f16 {
		Self::from("0")
	}

	pub fn from(n: &str) -> f16 {
		let result: f16 = f16 {
			bits: Vec::<u8>::new(),
		};
		let input: Vec<char> = n.chars().collect();

		result
	}
}

impl ops::Add<&f16> for &f16 {
	type Output = f16;

	fn add(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl ops::Sub<&f16> for &f16 {
	type Output = f16;

	fn sub(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl ops::Mul<&f16> for &f16 {
	type Output = f16;

	fn mul(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl ops::Div<&f16> for &f16 {
	type Output = f16;

	fn div(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl ops::Rem<&f16> for &f16 {
	type Output = f16;

	fn rem(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl ops::Shl<&f16> for &f16 {
	type Output = f16;

	fn shl(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl ops::Shr<&f16> for &f16 {
	type Output = f16;

	fn shr(self, x: &f16) -> f16 {
		f16::new()
	}
}

impl cmp::PartialEq for f16 {
	fn eq(&self, x: &f16) -> bool {
		false
	}
}

impl cmp::PartialOrd for f16 {
	fn partial_cmp(&self, x: &f16) -> Option<cmp::Ordering> {
		Some(cmp::Ordering::Less)
	}
}
