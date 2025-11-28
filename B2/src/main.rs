fn main() {
	let mut input = std::env::args().skip(1);
	println![
		"\"{}\"",
		lcs(&input.next().expect("No input provided"), &input.next().expect("No input provided"))
	];
}

fn lcs(s1 : &str, s2 : &str) -> String {
	let (a1, a2) : (&[u8], &[u8]) = (&s1.as_bytes(), &s2.as_bytes());
	let mut a : Vec<Vec<usize>> = vec![vec![0; a2.len() + 1]; a1.len() + 1];
	let (mut i, mut j) : (usize, usize) = (0, 0);
	let mut s : String = String::new();

	for i in (0..a1.len()).rev() {
		for j in (0..a2.len()).rev() {
			a[i][j] = if a1[i] == a2[j] {
				1 + a[i + 1][j + 1]
			}
			else {
				usize::max(a[i + 1][j], a[i][j + 1])
			}
		}
	}

	while a[i][j] != 0 {
		match usize::cmp(&a[i + 1][j], &a[i][j + 1]) as i8 {
			1 => i = i + 1,
			-1 => j = j + 1,
			_ => {
				s.push(a1[i] as char);
				(i, j) = (i + 1, j + 1);
			}
		}
	}

	s
}
