use std::fs::File;
use std::io::{BufReader,BufRead};

fn try_and_exit(mut jump_list: Vec<i32>) -> u32 {
	let mut steps: u32 = 0;
	let mut curr_idx: i32 = 0;
	loop {
		steps += 1;
		if curr_idx + jump_list[curr_idx as usize] >= jump_list.len() as i32 {
			return steps;
		}
		let curr_elem = jump_list[curr_idx as usize];
		if curr_elem >= 3 {
			jump_list[curr_idx as usize] -= 1;
		} else {
			jump_list[curr_idx as usize] += 1;
		}
		curr_idx += curr_elem;
	}
}

fn main() {
	let file = File::open("day5input").unwrap();
	let mut jump_list: Vec<i32> = Vec::new();
	for line in BufReader::new(file).lines() {
		jump_list.push(line.unwrap().parse::<i32>().unwrap());
	}
	println!("{:?}", try_and_exit(jump_list));
}