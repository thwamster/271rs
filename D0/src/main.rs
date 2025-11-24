use serde_json::{json, Map, Value};
use std::fs;

fn main() {
	let he : String = fs::read_to_string("helium.json").expect("File not Found");
	dbg!(&he);

	let json : Value = serde_json::from_str(&he).unwrap();
	dbg!(&json);

	let mut ne : Map<String, Value> = Map::new();
	ne.insert(String::from("symbol"), json!["Ne"]);
	ne.insert(String::from("phase_stp"), json!["gas"]);
	ne.insert(String::from("group"), json![18]);
	ne.insert(String::from("period"), json![2]);

	let mut bp : Map<String, Value> = Map::new();
	bp.insert(String::from("K"), json![27.104]);
	bp.insert(String::from("C"), json![-246.046]);
	bp.insert(String::from("F"), json![-410.883]);
	ne.insert(String::from("boiling_point"), json![bp]);

	dbg!(&ne);
}
