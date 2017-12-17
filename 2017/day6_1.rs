use std::collections::HashMap;
fn biggest_idx (banks: &Vec<i32>) -> usize {
	let mut maximal_index = 0;
	let biggest: &i32 = banks.iter().max().unwrap();
	for elem in banks {
		if elem == biggest {
			break;
		}
		maximal_index += 1;
	}
	maximal_index
}

fn main() {
	let mut banks: Vec<i32> = vec![4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5];
	// let mut banks: Vec<i32> = vec![0, 2, 7, 0];
	let banks_len = banks.len();
	let mut iters = 0;
	let mut seen = HashMap::new();
	while !seen.contains_key(&banks) {
		seen.insert(banks.clone(), iters);
		let max_idx = biggest_idx(banks.as_ref());
		let mem_in_bank = banks[max_idx];
		banks[max_idx] = 0;
		for j in 0..mem_in_bank as usize {
			banks[(max_idx + j + 1) % banks_len] += 1;
		}
		iters += 1;
	}
	println!("{:?}, count: {:?}", iters, iters - seen.get(&banks).unwrap());
}