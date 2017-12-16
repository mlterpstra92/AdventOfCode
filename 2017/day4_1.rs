use std::fs::File;
use std::io::{BufReader,BufRead};

fn verify_password(passphrase: String) -> bool {
	let mut split: Vec<&str> = passphrase.split(" ").collect();
	split.sort();
	let mut dedupped: Vec<&str> = split.clone();
	dedupped.dedup();
	return split.len() == dedupped.len()
}

fn main() {
	let file = File::open("day4input").unwrap();
	let mut num_valid = 0;
	for line in BufReader::new(file).lines() {
		match verify_password(line.unwrap()) {
			false => num_valid += 0,
			true => num_valid += 1
		}
	}
	println!("{:?}", num_valid);
}