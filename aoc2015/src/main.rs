use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    println!("Hello, world!");
    let input = match load_input_single_line("../../inputs/day01/input.txt") {
    	Ok(val) => val,
		Err(_) => {
			println!("NO FILE");
			return;
		},
    };

    let (part1, part2) = day01_solution(input);
    println!("Day 1 ---- Part1: {}, Part2: {}", part1, part2);
}


fn load_input_single_line(file_name: &str) -> Result<(String)> {
	let file = File::open(file_name)?;
	let mut test = String::new();

    for line in BufReader::new(file).lines() {
        test += &line?;
    }

    Ok(test)
}

fn day01_solution(input: String) -> (i32, u32) {
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