pub mod solution {
	use std::collections::HashMap;
	
	pub fn run() -> (i64, i64) {
		let _input = match crate::loadinput::file::load_input_multi_line("../../inputs/day18/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				return (0,0);
			},
		};

		//(part1_2(&_input, 100, 100, false),part1_2(&_input, 100, 100, false))
		(0,part1_2(&_input, 100, 100, true))
	}


	pub fn part1_2(input: &Vec<(String)>, size: i64, steps: usize, part2: bool) -> i64 {
		// just storing the cells which are actually on
		// Doing so through a map. This takes about 5 seconds to process
		// so i'm curious if this will be faster using a 2d Vec. or hard coding a 100x100 array

		//coords: &HashMap<(usize,usize), bool>
		let mut coords = HashMap::new();

		let mut y = 0;
		for s in input {
			let mut x = 0;
			for c in s.chars() {
				if c == '#' {
					coords.insert((x,y), true);
				}
				x += 1;
			}

			y += 1;
		}

		if (part2) {
			coords.insert((0,0), true);
			coords.insert((0,size-1), true);
			coords.insert((size-1, 0), true);
			coords.insert((size-1,size-1), true);
		}
		
		/*
    	A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
    	A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
		*/
		let mut total = 0;

		for _ in 0..steps {
			total = 0;

			let mut new_coords = HashMap::new();

			for x in 0..size {
				for y in 0..size {
					let cells = [(x-1, y-1),(x, y-1),(x+1, y-1),(x-1,y),(x+1,y),(x-1,y+1),(x,y+1),(x+1,y+1)];
					let mut sub_total = 0;
					let mut is_on = false;
					match coords.get(&(x,y)) {
						Some(_) => is_on = true,
						None => is_on = false
					}


					for cell in cells.iter() {
						if (cell.0 < 0 || cell.0 >= size || cell.1 < 0 || cell.1 >= size) {
							continue;
						}

						match coords.get(&(cell.0,cell.1)) {
							Some(_) => sub_total +=1,
							None => sub_total += 0
						}
					}

					if (is_on && (sub_total == 2 || sub_total == 3)) 
					|| (is_on == false && sub_total == 3) {
						total += 1;
						new_coords.insert((x,y), true);
					}
				}
			}

			coords = new_coords;
			//top corners are jammed on
			if (part2) {
				if coords.insert((0,0), true) == None {
					total += 1;
				}
				if coords.insert((0,size-1), true) == None {
					total += 1;
				}
				if coords.insert((size-1,0), true) == None {
					total += 1;
				}
				if coords.insert((size-1,size-1), true) == None {
					total += 1;
				}
			}
		}

		//print_grid(&coords, size);


		total
	}

	fn print_grid(coords: &HashMap<(i64,i64), bool>, size: i64) {
		for y in 0..size {
			let mut row = "".to_string();
			for x in 0..size {
				match coords.get(&(x,y)) {
					Some(_) => row.push_str("#"),
					None => row.push_str(".")
				}
			}
			println!("{:?}", row);	
		}
	}

	
	#[test]
	fn test_part_1_2() {
		let initial = &vec![
			".#.#.#".to_string(),
			"...##.".to_string(),
			"#....#".to_string(),
			"..#...".to_string(),
			"#.#..#".to_string(),
			"####..".to_string()
		];

		//let mut coords = HashMap::new();
		//coords.insert((0,0),true);

		let p1 = part1_2(&initial, 6,4, false);
		let p2 = part1_2(&initial, 6,5, true);

		assert_eq!(p1, 4);
		assert_eq!(p2, 17);
		//assert_eq!(p2, 689);
	}

}

