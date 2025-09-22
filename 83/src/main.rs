#![allow(non_snake_case)]

mod hash;

use num::Integer;
use num::Num;
use num::Signed;
use num::bigint::BigInt as i;
use num::bigint::Sign;
use std::fs::File;
use std::io::Read;

fn main() {
	let mut s = String::new();
	/*
	std::io::stdin()
		.read_line(&mut s)
		.expect("Failed to read line");
	s.truncate(s.len() - 1);
	*/
	s.push_str("Hello, world!");

	// let n: i = get_random();
	let n: i = i::from_str_radix(
		"47379675103498394144858916095175689779086087640336534911165206022228115974270",
		10,
	)
	.unwrap();

	dbg!(s.clone());
	dbg!(n.clone());

	ecdsa(s.as_str(), n);
}

fn ecdsa(input: &str, seed: i) {
	let p: i = (i::from(1) << 255) - 19;
	let base: (i, i) = (
		i::from_str_radix(
			"15112221349535400772501151409588531511454012693041857206046113283949847762202",
			10,
		)
		.unwrap(),
		i::from_str_radix(
			"46316835694926478169428394003475163141307993866256225615783033603165251855960",
			10,
		)
		.unwrap(),
	);

	dbg!(p.clone());
	dbg!(base.clone());

	let a: i = i::from(-1);
	let d: i = find_positive_modulus(
		i::from(-121665) * find_mod_inverse(i::from(121666), p.clone()).unwrap(),
		p.clone(),
	);
	let x0: i = base.clone().0;
	let y0: i = base.clone().1;

	dbg!(a.clone());
	dbg!(d.clone());
	dbg!(x0.clone());
	dbg!(y0.clone());

	let private_key: i = seed;
	let public_key: (i, i) = apply_double_and_add_method(
		base.clone(),
		private_key.clone(),
		a.clone(),
		d.clone(),
		p.clone(),
	);

	dbg!(private_key.clone());
	dbg!(public_key.clone());

	let message: i = text_to_int(input);
	let r: i = hashing(
		format!(
			"{}{}",
			hashing(message.to_str_radix(10).as_str()).to_str_radix(10),
			message
		)
		.as_str(),
	) % p.clone();
	let R: (i, i) =
		apply_double_and_add_method(base.clone(), r.clone(), a.clone(), d.clone(), p.clone());
	let mut h: i = hashing(
		(R.clone().0 + public_key.clone().0 + message.clone())
			.to_str_radix(10)
			.as_str(),
	) % p.clone();
	let s: i = r.clone() + h.clone() * private_key;

	dbg!(message.clone());
	dbg!(r.clone());
	dbg!(R.clone());
	dbg!(h.clone());
	dbg!(s.clone());

	h = hashing(
		(R.clone().0 + public_key.clone().0 + message)
			.to_str_radix(10)
			.as_str(),
	) % p.clone();

	let P1: (i, i) =
		apply_double_and_add_method(base.clone(), s.clone(), a.clone(), d.clone(), p.clone());
	let P2: (i, i) = point_addition(
		R.clone(),
		apply_double_and_add_method(
			public_key.clone(),
			h.clone(),
			a.clone(),
			d.clone(),
			p.clone(),
		),
		a.clone(),
		d.clone(),
		p.clone(),
	);

	dbg!(P1.clone());
	dbg!(P2.clone());

	if P1.0 == P2.0 && P1.1 == P2.1 {
		println!("The signature is valid.");
	} else {
		println!("Signature violation detected");
	}
}

fn find_positive_modulus(a: i, p: i) -> i {
	let mut n: i = a.clone();

	if a < i::ZERO {
		n = (a.clone() + p.clone() * (i::abs(&a) / p.clone()) + p.clone()) % p.clone()
	}

	n
}

fn text_to_int(text: &str) -> i {
	i::from_bytes_be(Sign::Plus, text.as_bytes())
}

fn gcd(mut a: i, mut b: i) -> i {
	while a != i::ZERO {
		(a, b) = (b.clone() % a.clone(), a.clone());
	}

	b
}

fn find_mod_inverse(mut a: i, m: i) -> Option<i> {
	if a < i::ZERO {
		a = (a.clone() + m.clone() * (i::abs(&a) / m.clone()) + m.clone()) % m.clone();
	}

	if gcd(a.clone(), m.clone()) != i::from(1) {
		return Option::None;
	}

	let (mut u1, mut u2, mut u3): (i, i, i) = (i::from(1), i::from(0), a.clone());
	let (mut v1, mut v2, mut v3): (i, i, i) = (i::from(0), i::from(1), m.clone());

	while v3 != i::from(0) {
		let q = u3.clone() / v3.clone();
		(v1, v2, v3, u1, u2, u3) = (
			(u1.clone() - q.clone() * v1.clone()),
			(u2.clone() - q.clone() * v2.clone()),
			(u3.clone() - q.clone() * v3.clone()),
			v1,
			v2,
			v3,
		);
	}
	Some(u1.clone().mod_floor(&m))
}

fn apply_double_and_add_method(P: (i, i), k: i, a: i, d: i, md: i) -> (i, i) {
	let mut addition_point: (i, i) = (P.clone().0, P.clone().1);

	let k_as_binary: String = format!("{:b}", k);

	for (i, c) in k_as_binary.chars().enumerate() {
		if i == 0 {
			continue;
		}

		addition_point = point_addition(
			addition_point.clone(),
			addition_point.clone(),
			a.clone(),
			d.clone(),
			md.clone(),
		);
		if c == '1' {
			addition_point = point_addition(
				addition_point.clone(),
				P.clone(),
				a.clone(),
				d.clone(),
				md.clone(),
			);
		}
	}

	addition_point
}

fn point_addition(P: (i, i), Q: (i, i), a: i, d: i, md: i) -> (i, i) {
	let (x1, y1, x2, y2): (i, i, i, i) = (P.clone().0, P.clone().1, Q.clone().0, Q.clone().1);

	let x3: i = (((x1.clone() * y2.clone() + y1.clone() * x2.clone()) % md.clone())
		* find_mod_inverse(
			1 + d.clone() * x1.clone() * x2.clone() * y1.clone() * y2.clone(),
			md.clone(),
		)
		.unwrap())
		% md.clone();
	let y3: i = (((y1.clone() * y2.clone() - a.clone() * x1.clone() * x2.clone()) % md.clone())
		* find_mod_inverse(
			1 - d.clone() * x1.clone() * x2.clone() * y1.clone() * y2.clone(),
			md.clone(),
		)
		.unwrap())
		% md.clone();

	(x3, y3)
}

fn get_random() -> i {
	let mut buf = [0u8; 32];
	File::open("/dev/urandom")
		.unwrap()
		.read_exact(&mut buf)
		.unwrap();
	i::from_bytes_be(Sign::Plus, &buf)
}

fn hashing(message: &str) -> i {
	i::from_str_radix(hash::hash(message).as_str(), 16).expect("Error")
}
