#![allow(unused)]
mod num;

use crate::num::Integer as i;

fn main() {
	let a: i = i::from("123");
	let b: i = i::from("-567");
	let c: i = i::from("789");
	let d: i = i::from("0xABC");
	let e: i = i::from("-0xDEF");

	dbg!(&a);
	dbg!(&b);
	dbg!(&c);
	dbg!(&d);
	dbg!(&e);
	test(&a, &b);
}

fn test(a: &i, b: &i) {
	dbg!(a + b);
	dbg!(a - b);
	dbg!(a * b);
	dbg!(a / b);
	dbg!(a % b);
	dbg!(a << b);
	dbg!(a >> b);
	dbg!(*a == *b);
	dbg!(*a > *b);
	dbg!(*a < *b);
}
