fn main() {
	let f : Vec<char> = read_file(1);
	let g : Vec<char> = read_file(2);

	dbg!(f, g);
}

fn read_file(n : usize) -> Vec<char> {
	std::fs::read_to_string(std::env::args().nth(n).expect("No path provided"))
		.expect("File not found")
		.chars()
		.collect()
}

fn sets_to_shorthand(i : usize, j : usize, ln : Vec<usize>, rn : Vec<usize>) -> String {
	fn side(i : usize, n : Vec<usize>) -> String {
		return match n.len() {
			// I couldn't print a `usize` so I turned them into `u64`'s and hoped for the best.
			0 => format!("{}", i as u64),
			1 => format!("{}", n[0]),
			// By the way - those vectors are backwards! Think about why they would be!
			_ => format!("{},{}", n[n.len() - 1] as u64, n[0] as u64)
		};
	}

	let letter = match (&ln.len(), &rn.len()) {
		(0, _) => "a",
		(_, 0) => "d",
		(..) => "c"
	};

	return format!("{}{}{}", side(i, ln), letter, side(j, rn));
}
