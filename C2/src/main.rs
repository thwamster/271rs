fn main() {
	print!["{}", diff(&read_file(1), &read_file(2))];
}

fn diff(v1 : &Vec<String>, v2 : &Vec<String>) -> String {
	let mut v : Vec<Vec<usize>> = vec![vec![0; v2.len() + 1]; v1.len() + 1];
	let (mut n1, mut n2) : (Vec<(usize, String)>, Vec<(usize, String)>) = (vec![], vec![]);
	let (mut i, mut j) : (usize, usize) = (0, 0);
	let (mut k, mut l) : (usize, usize) = (1, 1);
	let mut s : String = String::new();

	for i in (0..v1.len()).rev() {
		for j in (0..v2.len()).rev() {
			v[i][j] = if v1[i] == v2[j] {
				1 + v[i + 1][j + 1]
			}
			else {
				usize::max(v[i + 1][j], v[i][j + 1])
			}
		}
	}

	while i < v1.len() || j < v2.len() {
		if check_match(&v1, &v2, i, j) {
			if check_whitespace(&v1, &v2, i, j) {
				(n1, k) = check_first(n1, k, &v1, i);
				(n2, l) = check_first(n2, l, &v2, j);
				(i, j) = (i + 1, j + 1);
				continue;
			}

			if !n1.is_empty() || !n2.is_empty() {
				(n1, n2) = (check_last(n1), check_last(n2));
				s = format!["{s}{}", sets(&n1, &n2, k, l)];
				(_, _) = (n1.clear(), n2.clear());
			}

			(i, j, k, l) = (i + 1, j + 1, i + 2, j + 2);
			continue;
		}

		if j >= v2.len() || (i < v1.len() && v[i + 1][j] > v[i][j + 1]) {
			(n1, k) = check_first(n1, k, &v1, i);
			i += 1;
		}
		else {
			(n2, l) = check_first(n2, l, &v2, j);
			j += 1;
		}
	}

	if !n1.is_empty() || !n2.is_empty() {
		s = format!["{s}{}", sets(&n1, &n2, k, l)];
	}

	s
}

fn check_match(v1 : &Vec<String>, v2 : &Vec<String>, i : usize, j : usize) -> bool {
	i < v1.len() && j < v2.len() && v1[i] == v2[j]
}
fn check_whitespace(v1 : &Vec<String>, v2 : &Vec<String>, i : usize, j : usize) -> bool {
	i + 2 < v1.len()
		&& j + 2 < v2.len()
		&& v1[i].trim().is_empty()
		&& v2[j].trim().is_empty()
		&& v1[i + 1] != v2[j + 1]
		&& v1[i + 2] != v2[j + 2]
}

fn check_first(
	mut n : Vec<(usize, String)>, mut kl : usize, v : &Vec<String>, ij : usize
) -> (Vec<(usize, String)>, usize) {
	if n.is_empty() && v[ij].trim().is_empty() {
		kl += 1;
	}
	else {
		n.push((ij + 1, v[ij].clone()));
	}
	(n, kl)
}

fn check_last(mut n : Vec<(usize, String)>) -> Vec<(usize, String)> {
	if !n.is_empty() && n.last().unwrap().1.trim().is_empty() {
		n.pop();
	}
	n
}

fn sets(n1 : &Vec<(usize, String)>, n2 : &Vec<(usize, String)>, i : usize, j : usize) -> String {
	let l : &str = match (&n1.len(), &n2.len()) {
		(0, _) => "a",
		(_, 0) => "d",
		(..) => "c"
	};

	let (s1, s2) : (String, String) = (lines(&n1, "< "), lines(&n2, "> "));
	let s : String = if n1.is_empty() {
		s2
	}
	else if n2.is_empty() {
		s1
	}
	else {
		s1 + "---\n" + &s2
	};

	format!["{}{l}{}\n{s}", &side(n1, i), &side(n2, j),]
}

fn side(n : &Vec<(usize, String)>, i : usize) -> String {
	match n.len() {
		0 => format!("{}", i),
		1 => format!("{}", n[0].0),
		_ => format!("{},{}", n[0].0, n[n.len() - 1].0)
	}
}

fn lines(v : &Vec<(usize, String)>, p : &str) -> String {
	v.iter().map(|(_, s)| format!("{p}{s}")).collect::<Vec<_>>().join("\n") + "\n"
}

fn read_file(n : usize) -> Vec<String> {
	std::fs::read_to_string(std::env::args().nth(n).expect("No path provided"))
		.expect("File not found")
		.lines()
		.map(String::from)
		.collect()
}
