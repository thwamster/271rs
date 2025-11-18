fn main() {
	let mut o: Vec<char> = vec!['<', '~'];

	for i in std::fs::read(std::env::args().nth(1).expect("No path provided"))
		.expect("File not found")
		.chunks(4)
	{
		let mut n: u32 = 0;
		for j in 0..i.len() {
			n += (i[j] as u32) << ((3 - j) * 8);
		}

		let mut v: Vec<char> = vec![];
		for _ in 0..5 {
			v.push(((n % 85 + 33) as u8) as char);
			n /= 85;
		}

		for j in ((4 - i.len())..5).rev() {
			o.push(v[j]);
		}
	}

	o.extend(['~', '>']);

	for i in 0..o.len() {
		if i != 0 && i % 80 == 0 {
			println!();
		}
		print!("{}", o[i]);
	}
}
