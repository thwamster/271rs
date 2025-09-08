use colored_text::Colorize;
use rand::seq::IteratorRandom;
use std::fs;
use std::io;

fn main() {
	println!("\nGET 6 CHANCES TO GUESS A 5-LETTER WORD");

	play();

	let mut s: String;
	while {
		println!("PLAY AGAIN? (y/n)");

		s = get_input().to_lowercase();
		s != "n"
	} {
		if s == "y" {
			play();
		} else {
			println!("{}", "INVALID INPUT".white().on_red());
		}
	}
	
	println!("\nTHANKS FOR PLAYING TODAY!");
}

fn play() {
	let answer: String = get_answer();
	let mut log: String = format!("\n{}", "WORDLE".black().on_white().underline());
	let mut i: u64 = 0;

	println!("{}", log);

	while i <= 5 {
		i += 1;

		let mut guess: String;

		while {
			guess = get_input().to_uppercase();
			let mut b = false;
			if !is_valid_word(guess.clone()) {
				println!("{}", "NOT A WORD".white().on_red());
				b = true;
			} else if !is_valid_word_in_list(guess.clone()) {
				println!("{}", "NOT IN WORD LIST".white().on_red());
				b = true;
			}
			b
		} {
			println!("{}", log);
		}

		log += &get_results(answer.clone(), guess.clone());
		println!("{}", log);

		if guess == answer {
			let statement = match i {
				1 => "GENIUS!",
				2 => "MAGNIFICENT!",
				3 => "IMPRESSIVE!",
				4 => "SPLENDID!",
				5 => "GREAT!",
				6 => "PHEW!",
				_ => "",
			};

			println!("{}", statement.black().on_white());
			return;
		}
	}
	println!("{}", answer.black().on_white());
}

fn is_valid_word_in_list(s: String) -> bool {
	fs::read_to_string("list.txt")
		.map(|l| l.lines().any(|word| s == word))
		.unwrap()
}

fn is_valid_word(s: String) -> bool {
	s.len() == 5 && s.chars().all(|c| c.is_alphabetic())
}

fn get_input() -> String {
	let mut s = String::new();
	io::stdin().read_line(&mut s).expect("Failed to read line");
	return s.trim().to_string();
}

fn get_answer() -> String {
	fs::read_to_string("closed_list.txt")
        .expect("Failed to read file")
        .lines()
        .choose(&mut rand::rng())
        .unwrap()
        .to_string()
}

fn get_results(s1: String, s2: String) -> String {
	let mut result = format!("\n");

	let mut answer: Vec<char> = s1.chars().collect();
	let guess: Vec<char> = s2.chars().collect();
	let mut n = vec![0; 5];

	for i in 0..5 {
		if answer[i] == guess[i] {
			n[i] = 2;
			answer[i] = '\0';
		}
	}

	for i in 0..5 {
		let o = answer.iter().position(|&c| c == guess[i]);
		if o.is_some() {
			answer[o.unwrap()] = '\0';
			n[i] = 1;
		}
	}

	for i in 0..5 {
		if n[i] == 0 {
			result.push_str(&guess[i].white().on_black().to_string());
		} else if n[i] == 1 {
			result.push_str(&guess[i].white().on_yellow().to_string());
		} else {
			result.push_str(&guess[i].white().on_green().to_string());
		}
	}

	return result;
}
