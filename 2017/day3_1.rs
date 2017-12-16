enum Direction {
	Right,
	Left,
	Up, 
	Down
}

fn get_distance(num: i32) -> Option<i32> {
	if num < 1 {
		return None
	} else if num == 1 {
		return Some(0)
	} else {

		// Find the largest square smaller than the input
		let mut i = 1;
		let mut curr_val = 0;
		loop {
			let sq = i * i;
			if sq > num {
				i -= 2;
				curr_val = i * i;
				break;
			}
			i += 2
		}

		if curr_val == num { 
			return Some(i - 1);
		}
		// We now know our coordinate, what the arm length of the solution is on
		// and we can only walk right
		let (mut x, mut y) = (i / 2, -i / 2);
		// Length of an arm is 2 * limit + 1, so maximal index equals limit
		let limit = (i + 2) / 2;
		let mut direction: Direction = Direction::Right;
		loop {
			// Once found, return the manhattan distance
			if curr_val == num {
				return Some(x.abs() + y.abs());
			}
			curr_val += 1;

			// Correct direction if going out of bounds
			match direction {
				Direction::Up => {
					if y == limit {
						direction = Direction::Left;
					}
				},
				Direction::Left => {
					if x == -limit {
						direction = Direction::Down;
					}
				},
				Direction::Right => {
					if x == limit {
						direction = Direction::Up;
					}
				},
				Direction::Down => {
					if y == -limit {
						direction = Direction::Right;
					}
				}				
			}

			// Take a step
			match direction {
				Direction::Up    => y += 1,
				Direction::Down  => y -= 1,
				Direction::Left  => x -= 1,
				Direction::Right => x += 1
			}
		}

	}
}

fn main () {
	let inputs = vec![1, 9, 12, 23, 17, 21, 25, 1024, 368078];
	for input in inputs {
		println!("{:?} {:?}", input, get_distance(input));
	}
}