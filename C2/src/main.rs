fn main() {
	println!["{}", diff(&read_file(1), &read_file(2))];
}

fn diff(v1 : &Vec<u8>, v2 : &Vec<u8>) -> String {
	let mut v : Vec<Vec<usize>> = vec![vec![0; v2.len() + 1]; v1.len() + 1];
	let (mut i, mut j) : (usize, usize) = (0, 0);
	let mut s1 : String = String::new();
	let mut s2 : String = String::new();

	// finds LCS between V1 and V2 and stores it in S1
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
			s2.push(v1[i] as char);
			(i, j) = (i + 1, j + 1);
		}
		else if v[i + 1][j] >= v[i][j + 1] {
			i += 1;
		}
		else {
			j += 1;
		}
	}

	s1
}

fn side(i : usize, n : Vec<usize>) -> String {
	match n.len() {
		0 => format!("{}", i),
		1 => format!("{}", n[0]),
		_ => format!("{}, {}", n[n.len() - 1], n[0])
	}
}

fn sets_to_shorthand(i : usize, j : usize, n1 : Vec<usize>, n2 : Vec<usize>) -> String {
	let letter = match (&n1.len(), &n2.len()) {
		(0, _) => "a",
		(_, 0) => "d",
		(..) => "c"
	};

	format!("{}{}{}", side(i, n1), letter, side(j, n2))
}

fn read_file(n : usize) -> Vec<u8> {
	std::fs::read(std::env::args().nth(n).expect("No path provided")).expect("File not found")
}

fn read_line(n : usize) -> Vec<u8> {
	std::env::args().nth(n).expect("No path provided").into_bytes()
}
