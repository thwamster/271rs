use boxes::*;

fn main() {
	let mut s = Stack::<String>::new();
	dbg!(&s);

	s = s.push(String::from("0"));
	dbg!(&s);

	s = s.push(String::from("1"));
	dbg!(&s);

	let (mut s, x) = s.pop();
	dbg!(&x.unwrap());
	dbg!(&s);

	s = s.push(String::from("n"));
	dbg!(&s);
}
