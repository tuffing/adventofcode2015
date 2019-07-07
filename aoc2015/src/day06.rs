pub mod solution {
	use regex::Regex;
	
	pub fn run() -> (usize, usize) {	
		let input = match crate::loadinput::file::load_input_multi_line("../../inputs/day06/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		part_1_2_naive(input)
	}

	///
	/// This is an extremely naive solution. I'm just brute forcing a 2d array. rust is fast enough to make this bearable. 
	/// Without looking at what others would do, i think a better solution for part 1 would be to 
	/// calculate the sum and then take overlaps into account. 
	/// Part 2 would be trickier as we would have to keep track of the overlapped areas to make sure scores don't go below 0;
	/// @TODO give this a go at some point.
	///
	/// # Part 1
    ///	turn on 0,0 through 999,999 would turn on (or leave on) every light.
    ///	toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    ///	turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.
    ///
    /// Part 2
    /// The phrase turn on actually means that you should increase the brightness of those lights by 1.
    /// The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.
    /// The phrase toggle actually means that you should increase the brightness of those lights by 2.
	///
	pub fn part_1_2_naive(input: Vec<(String)>) -> (usize, usize) {
		struct Point {
			x: usize,
			y: usize,
		}
		let coords_regex = Regex::new(r"\d+", ).unwrap();

		//normally I prefer hashmaps and sets over 2d arrays
		//in this case, as we are dealing with a naive solution going over a millions upon millions of iterations
		//there is a substantive speed increase by using a 2d array instead (true look up of O(1) vs "O(1)" matters here.)
		let mut lights_part1 = [[false; 1000] ; 1000];
		let mut lights_part2 = [[0 as u8; 1000] ; 1000];

		for line in input {
			let coords: Vec<usize> = coords_regex.captures_iter(&line).map(|x| x[0].parse().unwrap()).collect();
			let coords = (Point { x: coords[0], y: coords[1] }, Point { x: coords[2], y: coords[3] });

			for x in coords.0.x..coords.1.x+1 {
				for y in coords.0.y..coords.1.y+1 {

					if line.starts_with("turn on") {
						lights_part1[x][y] = true;
						lights_part2[x][y] += 1;
					}
					else if line.starts_with("turn off") {
						lights_part1[x][y] = false;
						if lights_part2[x][y] > 0 {
							lights_part2[x][y] -= 1;
						}
					}
					else if line.starts_with("toggle") {
						lights_part1[x][y]  = !lights_part1[x][y];
						lights_part2[x][y] += 2;
					}
				}
			}
		}

		let mut total_part1 = 0;
		let mut total_part2 = 0;

		for x in 0..1000 {
			// loop in a loop as using iter..map..sum plays havoc with needing to go from u8 to usize
			// couldn't find a nice way to cast the numbers(or part 2). If we're looping. might as well do part 1 in the same loop
			for y in 0..1000 {
				if lights_part1[x][y] {
					total_part1 += 1;
				}

				total_part2 += usize::from(lights_part2[x][y]);
			}	
		}

		return (total_part1, total_part2)
	}



	#[test]
	fn test_part_1_2() {
		let (part1,part2) = part_1_2_naive(vec!["turn on 0,0 through 3,3".to_string(),
				"turn off 0,0 through 2,2".to_string(),
				"toggle 0,0 through 1,1".to_string()
			]
		);

		assert_eq!(part1, 11);
		assert_eq!(part2, 15);
	}

}

