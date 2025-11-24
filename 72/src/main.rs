#![allow(unused)]
mod num;

use crate::num::ix;

fn main() {
	let s : Vec<&str> = vec!["123", "-0xABC", "0b101", "123"];
	let v : Vec<ix> = s.iter().map(|n| ix::from(n)).collect();

	for i in 0..s.len() {
		dbg!(&s[i], &v[i]);
		println!();
	}

	for i in 0..s.len() {
		for j in (i + 1)..s.len() {
			test(v[i].clone(), v[j].clone());
			println!();
		}
	}
}

fn test(a : ix, b : ix) {
	dbg!(a.clone(), b.clone());
	dbg!(a.clone() + b.clone());
	dbg!(a.clone() - b.clone());
	dbg!(b.clone() - a.clone());
	dbg!(a.clone() * b.clone());
	dbg!(a.clone() / b.clone());
	dbg!(b.clone() / a.clone());
	dbg!(a.clone() % b.clone());
	dbg!(b.clone() % a.clone());
	dbg!(-a.clone());
	dbg!(-b.clone());
	dbg!(a == b);
	dbg!(a > b);
	dbg!(a < b);
}
