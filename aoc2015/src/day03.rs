pub mod solution {
	use std::collections::HashMap;


	pub fn run() -> (usize, usize) {	
		let input = match crate::loadinput::file::load_input_single_line("../../inputs/day03/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		(part1(&input),part2(&input))
	}

	fn coords(mut x: i32, mut y: i32, dir: char) -> (i32, i32) {
		if dir == '^' {
			y = y-1;
		}
		else if dir == '>' {
			x = x+1;
		}
		else if dir == 'v' {
			y = y+1;
		}
		else if dir == '<' {
			x = x-1;
		}

		(x,y)
	}

	/// Simply followings the directions and stores visited locations
	fn part1(input: &String) -> usize {
		//bad guess that part 2 would require knowing counts. could of gotten away with a set collection here
		let mut visited = HashMap::new();
		let mut x: i32 = 0;
		let mut y: i32 = 0;

		for c in input.chars() { 
			let (new_x,new_y) = coords(x,y,c);
			x = new_x;
			y = new_y;

			let count = visited.entry(format!("{},{}\n",x.to_string(),y.to_string())).or_insert(0);
    		*count += 1;
		}

		visited.len()
	}

	/// Identical to part 1 only we now track 2 paths at once, alternating between them
	fn part2(input: &String) -> usize {
		let mut visited = HashMap::new();
		let mut x_s: i32 = 0;
		let mut y_s: i32 = 0;
		let mut x_r: i32 = 0;
		let mut y_r: i32 = 0;

		//the puzzle counts our starting location as a visited point. 
		//in part 1 all of our examples and input happen to revisit it, no so on part 2
		visited.insert(format!("{},{}\n",x_s.to_string(),y_s.to_string()), 1);

		for (i,c) in input.chars().enumerate() {
			let mut x = x_s;
			let mut y = y_s;
			if i%2 == 1 {
				x = x_r;
				y = y_r;
			} 

			let (new_x,new_y) = coords(x,y,c);
			let count = visited.entry(format!("{},{}\n",new_x.to_string(),new_y.to_string())).or_insert(0);
    		*count += 1;

    		if i%2 == 0 {
    			x_s = new_x;
    			y_s = new_y;
    		}
    		else {
    			x_r = new_x;
    			y_r =new_y;
    		}
		}

		visited.len()
	}

	#[test]
	fn test_part_1() {
		let part1_sol = part1(&String::from("^>v<"));
		assert_eq!(part1_sol, 4);

		let part1_sol = part1(&String::from("^v^v^v^v^v"));
		assert_eq!(part1_sol, 2);
	}

	#[test]
	fn test_part_2() {
		let part2_sol = part2(&String::from("^>v<"));
		assert_eq!(part2_sol, 3);

		let part2_sol = part2(&String::from("^v"));
		assert_eq!(part2_sol, 3);

		let part2_sol = part2(&String::from("^v^v^v^v^v"));
		assert_eq!(part2_sol, 11);
	}

}