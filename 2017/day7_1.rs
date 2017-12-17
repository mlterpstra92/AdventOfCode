use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: i32,
    supports: HashSet<String>,
    parent: String
}

fn parse_file (file_name: &str) {
	let file = File::open(file_name).unwrap();
	let mut program_definitions: Vec<Program> = Vec::new();
	for line in BufReader::new(file).lines() {
		let l = line;
		let w = l.unwrap();
		let mut split = w.split_whitespace();
		let name = split.next().unwrap();
		let weight_str: String = split.next().unwrap().split(|x| x == '(' || x == ')').collect();
		let weight: i32 = weight_str.parse::<i32>().unwrap();
		let support_arrow = split.next();
		let mut supports: HashSet<String> = HashSet::new();
		match support_arrow {
			None => program_definitions.push(Program {
				name: name.to_string(),
				weight: weight,
				supports: HashSet::new(),
				parent: String::from("")
			}),
			Some(_) => {
				loop {
					let program_name = split.next();
					match program_name {
						None => break,
						Some(name) => supports.insert(name.replace(",", ""))
					};
				}
				program_definitions.push(Program {
					name: name.to_string(),
					weight: weight,
					supports: supports,
					parent: String::from("")
				})
			}

		}
	}

	let mut has_parent: HashSet<&String> = HashSet::new();
	for program in program_definitions.iter() {
		if program.supports.len() > 0 {
			for support_name in &program.supports {
				has_parent.insert(support_name);
			}	
		}
	}
	for program in program_definitions.iter() {
		if !has_parent.contains(&program.name) {
			println!("{:?}", program);
			break;
		}
	}
}

fn main () {
	parse_file("day7input");
}
