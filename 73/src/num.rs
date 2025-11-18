#![allow(non_camel_case_types)]
use std::cmp;
use std::fmt;
use std::ops;

pub struct ix {
	sign: u8,
	bytes: Vec<u8>,
}

impl ix {
	pub fn new() -> ix {
		ix {
			sign: 0,
			bytes: vec![0],
		}
	}

	pub fn one() -> ix {
		ix {
			sign: 0,
			bytes: vec![1],
		}
	}

	pub fn from(input: &str) -> ix {
		let mut result: ix = ix {
			sign: 0,
			bytes: Vec::<u8>::new(),
		};
		let mut nums: Vec<char> = input.chars().collect();
		let mut radix: u8 = 10;

		if nums.len() >= 1 && nums[0] == '-' {
			result.sign = 1;
			nums.remove(0);
		}

		if nums.len() >= 2 && nums[0] == '0' {
			let r = match nums[1] {
				'x' => Some(16),
				'd' => Some(10),
				'o' => Some(8),
				'b' => Some(2),
				_ => None,
			};

			if r.is_some() {
				radix = r.unwrap();
				nums.drain(0..2);
			}
		}

		if nums.len() == 0 {
			panic!("Invalid Input");
		}

		if radix & (radix - 1) == 0 {
			nums.reverse();

			for s in nums.chunks((8 / radix.ilog2()) as usize) {
				let mut n: u8 = 0;

				for (j, c) in s.iter().enumerate() {
					n += (c.to_digit(radix as u32).expect("Invalid Input")
						<< (j * radix.ilog2() as usize)) as u8;
				}

				result.bytes.push(n);
			}
		} else if radix == 10 {
			let mut byte: u8 = 0;
			let mut i: usize = 0;

			while nums.len() != 0 {
				if nums
					.last()
					.unwrap()
					.to_digit(radix as u32)
					.expect("Invalid Input")
					% 2 != 0
				{
					byte += 1 << i;
				}

				i += 1;

				if i >= 8 {
					result.bytes.push(byte);
					byte = 0;
					i = 0;
				}

				let mut now: u32;
				let mut next: u32 = 0;

				for c in nums.iter_mut() {
					let n: u32 = c.to_digit(radix as u32).expect("Invalid Input");

					now = next;
					next = 5;

					if n % 2 == 0 {
						next = 0;
					}

					*c = char::from_digit(((n / 2) as u32) + now, radix as u32)
						.expect("Invalid Input");
				}

				while nums.len() >= 1 && nums[0] == '0' {
					nums.remove(0);
				}
			}

			if i != 0 {
				result.bytes.push(byte);
			}
		}

		result.clone()
	}

	pub fn from_bytes(bytes: Vec<u8>) -> ix {
		ix {
			sign: 0,
			bytes: bytes,
		}
	}

	pub fn to(&self, radix: u8) -> String {
		let mut s: String = String::new();

		match radix {
			2 => {
				for byte in self.bytes.iter().rev() {
					s.push_str(format!("{:08b}", byte).as_str());
				}
			}
			16 => {
				for byte in self.bytes.iter().rev() {
					s.push_str(format!("{:02x}", byte).as_str());
				}
			}
			10 => {
				let mut digits: Vec<u8> = vec![0];
				let mut t: u16;

				for byte in self.bytes.iter().rev() {
					t = *byte as u16;
					for d in digits.iter_mut() {
						t = (*d as u16) * 256 + t;
						*d = (t % 10) as u8;
						t = t / 10;
					}

					while t > 0 {
						digits.push((t % 10) as u8);
						t /= 10;
					}
				}

				for d in digits.iter().rev() {
					s.push_str(format!("{}", *d).as_str());
				}
			}
			_ => panic!("Invalid input"),
		}

		s.trim_start_matches('0').to_string()
	}

	pub fn divmod(self, x: ix) -> (ix, ix) {
		let n: ix = self.abs();
		let d: ix = x.abs();
		let mut q: ix = ix::new();
		let mut r: ix = ix::new();
		let mut now: u8;
		let mut next: u8;

		if d.bytes.len() == 1 && d.bytes[0] == 0 {
			panic!("Undefined behavior");
		}

		q.bytes = vec![0; n.bytes.len()];

		for i in (0..n.bytes.len()).rev() {
			for j in (0..8).rev() {
				now = (n.bytes[i] >> j) & 1;

				for byte in r.bytes.iter_mut() {
					next = *byte >> 7;
					*byte <<= 1;
					*byte |= now;
					now = next;
				}
				if now != 0 {
					r.bytes.push(now);
				}

				if r >= d {
					r = r.clone() - d.clone();
					q.bytes[i] = q.bytes[i] | (1 << j);
				}
			}
		}

		q.sign = self.sign ^ x.sign;
		r.sign = self.sign ^ x.sign;

		(q.clone(), r.clone())
	}

	pub fn abs(&self) -> ix {
		let mut n: ix = self.clone();
		n.sign = 0;
		n
	}
}

impl Clone for ix {
	fn clone(&self) -> ix {
		let mut c: ix = ix {
			sign: self.sign,
			bytes: self.bytes.clone(),
		};

		if c.bytes.len() == 0 {
			c.bytes.push(0);
		}

		while c.bytes.len() > 1 && c.bytes.last() == Some(&0) {
			c.bytes.pop();
		}

		if c.bytes.len() == 1 && c.bytes[0] == 0 {
			c.sign = 0;
		}

		c
	}
}

impl fmt::Debug for ix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ix {{ sign: {}, bytes: [", self.sign)?;

		for (i, byte) in self.bytes.iter().enumerate() {
			write!(f, "{:08b}", byte)?;

			if i < self.bytes.len() - 1 {
				write!(f, ", ")?;
			}
		}

		write!(f, "], hex: {} }}", format!("{}", self))
	}
}

impl fmt::Display for ix {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.sign == 1 {
			write!(f, "-")?;
		}

		write!(f, "{}", self.to(10))
	}
}

impl ops::Add<ix> for ix {
	type Output = ix;

	fn add(self, x: ix) -> ix {
		let mut a: ix;
		let b: ix;
		let mut t: i16 = 0;
		let mut i: usize = 0;

		if self.abs().partial_cmp(&x.abs()).unwrap() != cmp::Ordering::Less {
			a = self;
			b = x;
		} else {
			a = x;
			b = self;
		}

		while (i < a.bytes.len() && i < b.bytes.len()) || t != 0 {
			if i >= a.bytes.len() {
				a.bytes.push(0);
			}

			t += a.bytes[i] as i16;

			if i < b.bytes.len() {
				if a.sign == b.sign {
					t += b.bytes[i] as i16;
				} else {
					t -= b.bytes[i] as i16;
				}
			}

			if t < 0 {
				a.bytes[i] = (t + 256) as u8;
				t = -1;
			} else if t >= 0 && t < 256 {
				a.bytes[i] = t as u8;
				t = 0;
			} else if t >= 256 {
				a.bytes[i] = (t - 256) as u8;
				t = 1;
			}

			i += 1;
		}

		a.clone()
	}
}

impl ops::Sub<ix> for ix {
	type Output = ix;

	fn sub(self, x: ix) -> ix {
		self + (-x)
	}
}

impl ops::Mul<ix> for ix {
	type Output = ix;

	fn mul(self, x: ix) -> ix {
		let mut n: ix = ix::new();
		let mut t: u16;
		let mut j: usize;

		n.bytes = vec![0; self.bytes.len() + x.bytes.len()];

		n.sign = self.sign ^ x.sign;

		for i in 0..self.bytes.len() {
			t = 0;
			j = 0;

			while j < x.bytes.len() || t != 0 {
				t += n.bytes[i + j] as u16;

				if j < x.bytes.len() {
					t += (self.bytes[i] as u16) * (x.bytes[j] as u16);
				}

				n.bytes[i + j] = (t % 256) as u8;
				t = t / 256;

				j += 1;
			}
		}

		n.clone()
	}
}

impl ops::Div<ix> for ix {
	type Output = ix;

	fn div(self, x: ix) -> ix {
		ix::divmod(self, x).0
	}
}

impl ops::Rem<ix> for ix {
	type Output = ix;

	fn rem(self, x: ix) -> ix {
		ix::divmod(self, x).1
	}
}

impl ops::Neg for ix {
	type Output = ix;

	fn neg(self) -> ix {
		let mut n: ix = self.clone();
		n.sign = 1 ^ n.sign;
		n
	}
}

impl cmp::PartialEq for ix {
	fn eq(&self, x: &ix) -> bool {
		self.sign == x.sign && self.bytes == x.bytes
	}
}

impl cmp::PartialOrd for ix {
	fn partial_cmp(&self, x: &ix) -> Option<cmp::Ordering> {
		match self.sign.cmp(&x.sign).reverse() {
			cmp::Ordering::Equal => {}
			o => return Some(o),
		}

		match self.bytes.len().cmp(&x.bytes.len()) {
			cmp::Ordering::Equal => {}
			o => return Some(o),
		}

		Some(self.bytes.iter().rev().cmp(x.bytes.iter().rev()))
	}
}
