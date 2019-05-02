pub mod solution {
	use std::fs::File;
	use std::io::{BufRead, BufReader, Result};
	use regex::Regex;
	use std::cmp;

	pub fn run() -> (i32, i32) {	
	    let input = match load_input_single_line("../../inputs/day02/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
   		 };

    	part_1_and_2(input)
	}

	pub fn part_1_and_2(input: Vec<(i32,i32,i32)>) -> (i32, i32) {
		let mut part1 = 0;
		let mut part2 = 0;
		for i in input {
			//part 1
		    let first = i.0 * i.1;
		    let second = i.0 * i.2;
		    let third = i.1 * i.2;
		    let mut min = cmp::min(first, second); 
		    min = cmp::min(min, third);

		    part1 += (2*first) + (2*second) + (2*third) + (min) ;

		    //part 2
		    let mut max = cmp::max(i.0, i.1);
		    max = cmp::max(max, i.2);
		    part2 += (i.0 * i.1 * i.2) + (i.0+i.1+i.2-max)*2;
		}

		(part1,part2)
	}

	fn load_input_single_line(file_name: &str) -> Result<(Vec<(i32, i32, i32)>)> {
		let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();

		let file = File::open(file_name)?;
		let mut rows: Vec<(i32, i32, i32)> = Vec::new();

		for line in BufReader::new(file).lines() {
			//test += &line?;
			let line = &line?;
			let nums = re.captures(line).unwrap();
			rows.push((nums[1].parse().unwrap(),nums[2].parse().unwrap(),nums[3].parse().unwrap()));
		}

		Ok(rows)
	}


	#[test]
	fn test_part_1_and_2() {
		let (part1, part2) = part_1_and_2(vec![(2,3,4)]);
		assert_eq!(part1, 58);
		assert_eq!(part2, 34);

		let (part1, part2) = part_1_and_2(vec![(1,1,10)]);
		assert_eq!(part1, 43);
		assert_eq!(part2, 14);
	}

}

