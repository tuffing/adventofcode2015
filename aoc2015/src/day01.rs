pub mod solution {
	use std::fs::File;
	use std::io::{BufRead, BufReader, Result};

	pub fn run() -> (i32, u32) {	
	    let input = match load_input_single_line("../../inputs/day01/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
   		 };

    	part_1_and_2(input)
	}

	pub fn part_1_and_2(input: String) -> (i32, u32) {
		let mut floor = 0;
		let mut part2 = 0;
		let mut count = 0;

		for c in input.chars() { 
			count += 1;

			if c == '(' {
				floor += 1;
			}
			else {
				floor -= 1;
			}

			if floor == -1 && part2 == 0 {
				part2 = count;
			}
		}

		(floor,part2)
	}

	fn load_input_single_line(file_name: &str) -> Result<(String)> {
		let file = File::open(file_name)?;
		let mut test = String::new();

		for line in BufReader::new(file).lines() {
			test += &line?;
		}

		Ok(test)
	}


	#[test]
	fn test_part_1_and_2() {
		let (part1, part2) = part_1_and_2(String::from("(()))"));
		assert_eq!(part1, -1);
		assert_eq!(part2, 5);
	}

}

