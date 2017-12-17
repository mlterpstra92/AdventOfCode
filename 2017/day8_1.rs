use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;
#[derive(Debug)]
enum Operation {
	Inc, 
	Dec
}

#[derive(Debug)]
enum Comp {
	LT,
	GT,
	LEQ,
	GEQ,
	NEQ,
	EQ
}

#[derive(Debug)]
struct Condition {
	register: String,
	comp: Comp,
	threshold: i32
}

#[derive(Debug)]
struct Program {
	register: String,
	op: Operation,
	amount: i32,
	condition: Condition
}

fn parse_condition (split: &mut std::str::SplitWhitespace) -> Condition {
	let _ = split.next();
	let register_name = split.next().unwrap();
	let operation: Comp = match split.next().unwrap() {
		"<" => Comp::LT,
		"<=" => Comp::LEQ,
		">" => Comp::GT,
		">=" => Comp::GEQ,
		"!=" => Comp::NEQ,
		_ => Comp::EQ,
	};
	let threshold = split.next().unwrap().parse::<i32>().unwrap();
	return Condition {
		register: register_name.to_string(),
		comp: operation,
		threshold: threshold
	};
}

fn parse_file (file_name: &str) -> Vec<Program> {
	let file = File::open(file_name).unwrap();
	let mut program_definitions: Vec<Program> = Vec::new();
	for l in BufReader::new(file).lines() {
		let line = l.unwrap();
		let mut split = line.split_whitespace();
		let register_name = split.next().unwrap();
		let op: Operation = match split.next().unwrap() {
			"inc" => Operation::Inc,
			_ => Operation::Dec
		};
		let amount = split.next().unwrap().parse::<i32>().unwrap();
		let condition: Condition = parse_condition(&mut split);
		program_definitions.push(Program {
			register: register_name.to_string(),
			op: op,
			amount: amount,
			condition: condition
		})
	}
	program_definitions
}

fn eval (clause: &Condition, val: i32) -> bool {
	match clause.comp {
		Comp::GT => return val > clause.threshold,
		Comp::GEQ => return val >= clause.threshold,
		Comp::LT => return val < clause.threshold,
		Comp::LEQ => return val <= clause.threshold,
		Comp::NEQ => return val != clause.threshold,
		_ => return val == clause.threshold
	}
}

fn main () {
	let mut registers: HashMap<String, i32> = HashMap::new();
	let conditions = parse_file("day8input");
	for condition in conditions.iter() {
		let register = &condition.register;
		if !registers.contains_key(register) {
			registers.insert(register.to_string(), 0);
		}
		let clause = &condition.condition;
		let clause_register = &clause.register;
		if !registers.contains_key(clause_register) {
			registers.insert(clause_register.to_string(), 0);
		}

		let current_value: i32 = *registers.get(register).unwrap();
		if eval(clause, registers[clause_register]) {
			match condition.op {
				Operation::Inc => registers.insert(register.to_string(), current_value + condition.amount),
				_ => registers.insert(register.to_string(), current_value - condition.amount)
			};
		}
	}
	
	let mut max_val = <i32>::min_value();
	let mut max_key = String::from("");
	for elem in registers.iter() {
		if elem.1 > &max_val {
			max_val = *elem.1;
			max_key = elem.0.to_string();
		}
	}
	println!("After: {:?} {:?}", max_key, max_val);
}