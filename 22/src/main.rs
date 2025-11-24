#![feature(let_chains)]

use colored_text::Colorize;
use std::{fs::File,
		  io::{stdin, Read}};

fn main() {
	println!("\nGET 6 CHANCES TO GUESS A 5-LETTER WORD");

	play();

	loop {
		println!("PLAY AGAIN? (y/n)");

		match get_input().to_lowercase().as_str() {
			"y" => play(),
			"n" => break,
			_ => println!("{}", "INVALID INPUT".white().on_red())
		}
	}

	println!("\nTHANKS FOR PLAYING TODAY!");
}

fn play() {
	let answer : String = get_answer();
	let mut log : String = format!("\n{}", "WORDLE".black().on_white().underline());

	println!("{}", log);

	for i in 0..6 {
		let mut guess : String;

		loop {
			guess = get_input().to_uppercase();

			if !is_valid_word(guess.clone()) {
				println!("{}", "NOT A WORD".white().on_red());
			}
			else if !is_valid_word_in_list(guess.clone()) {
				println!("{}", "NOT IN WORD LIST".white().on_red());
			}
			else {
				break;
			}

			println!("{}", log);
		}

		log += &get_results(answer.clone(), guess.clone());
		println!("{}", log);

		if guess == answer {
			let statement = match i {
				0 => "GENIUS!",
				1 => "MAGNIFICENT!",
				2 => "IMPRESSIVE!",
				3 => "SPLENDID!",
				4 => "GREAT!",
				5 => "PHEW!",
				_ => ""
			};

			println!("{}", statement.black().on_white());
			return;
		}
	}

	println!("{}", answer.black().on_white());
}

fn is_valid_word_in_list(s : String) -> bool {
	include_str!("list.txt").lines().any(|word| s == word)
}

fn is_valid_word(s : String) -> bool { s.len() == 5 && s.chars().all(|c| c.is_alphabetic()) }

fn get_input() -> String {
	let mut s = String::new();
	stdin().read_line(&mut s).expect("Failed to read line");
	s.trim().to_string()
}

fn get_answer() -> String {
	let l : Vec<&str> = include_str!("closed_list.txt").lines().collect();
	let i = get_random(l.len());
	l[i].to_string()
}

fn get_random(n : usize) -> usize {
	let mut buf = [0u8; 8];
	File::open("/dev/urandom").unwrap().read_exact(&mut buf).unwrap();
	usize::from_ne_bytes(buf) % n
}

fn get_results(s1 : String, s2 : String) -> String {
	let mut result = format!("\n");

	let mut answer : Vec<char> = s1.chars().collect();
	let guess : Vec<char> = s2.chars().collect();
	let mut n = vec![0; 5];

	for i in 0..5 {
		if answer[i] == guess[i] {
			answer[i] = '\0';
			n[i] = 2;
		}
	}

	for i in 0..5 {
		if n[i] == 0
			&& let Some(o) = answer.iter().position(|&c| c == guess[i])
		{
			answer[o] = '\0';
			n[i] = 1;
		}
	}

	for i in 0..5 {
		match n[i] {
			2 => result.push_str(&guess[i].white().on_green().to_string()),
			1 => result.push_str(&guess[i].white().on_yellow().to_string()),
			_ => result.push_str(&guess[i].white().on_black().to_string())
		}
	}

	result
}
