fn main() {
	let k: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
		.chars()
		.collect();

	let mut o: Vec<char> = Vec::<char>::new();
	for i in std::fs::read(std::env::args().nth(1).expect("No path provided"))
		.expect("File not found")
		.chunks(3)
	{
		match i.len() {
			3 => {
				o.push(k[(i[0] >> 2) as usize]);
				o.push(k[(((i[0] & 0b11) << 4) | (i[1] >> 4)) as usize]);
				o.push(k[(((i[1] & 0b1111) << 2) | (i[2] >> 6)) as usize]);
				o.push(k[(i[2] & 0b111111) as usize]);
			}
			2 => {
				o.push(k[(i[0] >> 2) as usize]);
				o.push(k[(((i[0] & 0b11) << 4) | (i[1] >> 4)) as usize]);
				o.push(k[((i[1] & 0b1111) << 2) as usize]);
				o.push(k[64]);
			}
			_ => {
				o.push(k[(i[0] >> 2) as usize]);
				o.push(k[((i[0] & 0b11) << 4) as usize]);
				o.push(k[64]);
				o.push(k[64]);
			}
		}
	}

	o.chunks(76)
		.for_each(|s| println!("{}", s.iter().collect::<String>()));
}
