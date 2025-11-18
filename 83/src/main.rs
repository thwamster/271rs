#![allow(dead_code, non_snake_case)]

mod hash;
mod num;

use crate::num::ix;
use std::fs::File;
use std::io::Read;

fn main() {
	/*
	let mut s = String::new();
	std::io::stdin()
		.read_line(&mut s)
		.expect("Failed to read line");
	s.truncate(s.len() - 1);
	*/
	let s = String::from("Hello, world!");

	let n: ix = get_random();
	// let n: ix = ix::from("47379675103498394144858916095175689779086087640336534911165206022228115974270");

	ecdsa(s.as_str(), n);
}

fn ecdsa(input: &str, seed: ix) {
	println!("Inputs");
	println!("Input: {}", input);
	println!("Seed: {}", seed);
	println!("\nConstants");

	let p: ix = ix::from("0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");
	println!("p: {}", p);

	let base: (ix, ix) = (
		ix::from("15112221349535400772501151409588531511454012693041857206046113283949847762202"),
		ix::from("46316835694926478169428394003475163141307993866256225615783033603165251855960"),
	);
	println!("Base: ({}, {})", base.0, base.1);
	println!("\nCurve");

	let a: ix = ix::from("-1");
	let d: ix = find_positive_modulus(
		ix::from("-121665") * find_mod_inverse(ix::from("121666"), p.clone()),
		p.clone(),
	);
	println!("a: {}", a);
	println!("d: {}", d);

	let x0: ix = base.clone().0;
	let y0: ix = base.clone().1;
	println!("x0: {}", x0);
	println!("y0: {}", y0);
	println!("\nKey Generation");

	let private_key: ix = seed;
	let public_key: (ix, ix) = apply_double_and_add_method(
		base.clone(),
		private_key.clone(),
		a.clone(),
		d.clone(),
		p.clone(),
	);
	println!("Private Key: {}", private_key);
	println!("Public Key: ({}, {})", public_key.0, public_key.1);
	println!("\nSigning");

	let message: ix = text_to_int(input);
	println!("Message: {}", message);

	let r: ix = hashing(
		(hashing(message.to(10).as_str()) + message.clone())
			.to(10)
			.as_str(),
	) % p.clone();
	println!("r: {}", r);

	let R: (ix, ix) =
		apply_double_and_add_method(base.clone(), r.clone(), a.clone(), d.clone(), p.clone());
	println!("R: ({}, {})", R.0, R.1);

	let h: ix = hashing(
		(R.clone().0 + public_key.clone().0 + message.clone())
			.to(10)
			.as_str(),
	) % p.clone();
	println!("h: {}", h);

	let s: ix = r.clone() + h.clone() * private_key;
	println!("s: {}", s);
	println!("\nVerification");

	let P1: (ix, ix) =
		apply_double_and_add_method(base.clone(), s.clone(), a.clone(), d.clone(), p.clone());
	println!("P1: ({}, {})", P1.0, P1.1);

	let P2: (ix, ix) = point_addition(
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
	println!("P2: ({}, {})\n", P2.0, P2.1);

	if P1.0 == P2.0 && P1.1 == P2.1 {
		println!("The signature is valid.");
	} else {
		println!("Signature violation detected.");
	}

	println!();
}

fn find_positive_modulus(a: ix, p: ix) -> ix {
	let mut n: ix = a.clone();

	if a < ix::new() {
		n = (a.clone() + p.clone() * (ix::abs(&a) / p.clone()) + p.clone()) % p.clone()
	}

	n
}

fn text_to_int(text: &str) -> ix {
	ix::from_bytes(text.as_bytes().to_vec().iter().rev().cloned().collect())
}

fn gcd(mut a: ix, mut b: ix) -> ix {
	while a != ix::new() {
		(a, b) = (b.clone() % a.clone(), a.clone());
	}

	b
}

fn find_mod_inverse(mut a: ix, m: ix) -> ix {
	if a < ix::new() {
		a = (a.clone() + m.clone() * (ix::abs(&a) / m.clone()) + m.clone()) % m.clone();
	}

	if gcd(a.clone(), m.clone()) != ix::one() {
		panic!("Undefined behavior");
	}

	let (mut u1, mut u2, mut u3): (ix, ix, ix) = (ix::one(), ix::new(), a.clone());
	let (mut v1, mut v2, mut v3): (ix, ix, ix) = (ix::new(), ix::one(), m.clone());

	while v3 != ix::new() {
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

	let mut n: ix = u1.clone() % m.clone();

	if n < ix::new() {
		n = n.clone() + m.clone();
	}

	n
}

fn apply_double_and_add_method(P: (ix, ix), k: ix, a: ix, d: ix, md: ix) -> (ix, ix) {
	let mut addition_point: (ix, ix) = (P.clone().0, P.clone().1);

	let k_as_binary: String = k.to(2);

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

fn point_addition(P: (ix, ix), Q: (ix, ix), a: ix, d: ix, md: ix) -> (ix, ix) {
	let (x1, y1, x2, y2): (ix, ix, ix, ix) = (P.clone().0, P.clone().1, Q.clone().0, Q.clone().1);

	let x3: ix = (((x1.clone() * y2.clone() + y1.clone() * x2.clone()) % md.clone())
		* find_mod_inverse(
			ix::one() + d.clone() * x1.clone() * x2.clone() * y1.clone() * y2.clone(),
			md.clone(),
		)) % md.clone();
	let y3: ix = (((y1.clone() * y2.clone() - a.clone() * x1.clone() * x2.clone()) % md.clone())
		* find_mod_inverse(
			ix::one() - d.clone() * x1.clone() * x2.clone() * y1.clone() * y2.clone(),
			md.clone(),
		)) % md.clone();

	(x3, y3)
}

fn get_random() -> ix {
	let mut buf = [0u8; 32];
	File::open("/dev/urandom")
		.unwrap()
		.read_exact(&mut buf)
		.unwrap();
	ix::from_bytes(buf.to_vec())
}

fn hashing(message: &str) -> ix {
	ix::from(&format!("0x{}", hash::hash(message).as_str()))
}
