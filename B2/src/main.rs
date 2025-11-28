fn main() {
	println!["\"{}\"", lcs(&read_line(1), &read_line(2))];
}

fn lcs(v1 : &Vec<u8>, v2 : &Vec<u8>) -> String {
	let mut v : Vec<Vec<usize>> = vec![vec![0; v2.len() + 1]; v1.len() + 1];
	let (mut i, mut j) : (usize, usize) = (0, 0);
	let mut s : String = String::new();

	for i in (0..v1.len()).rev() {
		for j in (0..v2.len()).rev() {
			v[i][j] = if v1[i] == '\0' as u8 || v2[j] == '\0' as u8 {
				0
			}
			else if v1[i] == v2[j] {
				1 + v[i + 1][j + 1]
			}
			else {
				usize::max(v[i + 1][j], v[i][j + 1])
			}
		}
	}

	while i < v1.len() && j < v2.len() {
		if v1[i] == v2[j] {
			s.push(v1[i] as char);
			(i, j) = (i + 1, j + 1);
		}
		else if v[i + 1][j] >= v[i][j + 1] {
			i += 1;
		}
		else {
			j += 1;
		}
	}

	s
}

fn read_line(n : usize) -> Vec<u8> {
	std::env::args().nth(n).expect("No path provided").into_bytes()
}
