use std::fs::File;
use std::io::{BufReader,BufRead};
use std::iter::Iterator;
use std::iter::FromIterator;
fn sort_string(x: &str) -> String {
	let mut chars: Vec<char> = x.chars().collect();
	chars.sort_by(|a, b| b.cmp(a));
	return String::from_iter(chars);
}

fn verify_password(passphrase: String) -> bool {
	let mut split: Vec<String> = passphrase.split(" ").map(|x| sort_string(x)).collect();
	split.sort();
	let mut dedupped: Vec<String> = split.clone();
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