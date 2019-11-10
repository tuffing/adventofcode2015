pub mod solution {
	pub fn run() -> (usize, usize) {
		(part1_2(3010,3019), 0)
	}


	pub fn part1_2(rows: usize, cols: usize) -> (usize) {
		/* The first code is 20151125. 
		After that, each code is generated by taking the previous one, multiplying it by 252533, 
		and then keeping the remainder from dividing that value by 33554393.*/
		let mut start_y = 1;
		let mut hit = false;
		let mut last = 20151125;

		//the puzzle shows a table but in reality we only need the last number.
		//the inner for loop acts as a ticker so that i know
		//when i'm on the right cell
		while !hit {
			for x in 0..start_y+1 {
				let y = start_y - x;

				last  = (last * 252533) % 33554393;

				if y+1 == rows && x+1 == cols {
					hit = true;
					break;
				}
			}

			start_y += 1;

			if start_y > cols * 10 {
				//just a safety check against an infinite loop
				println!("bailing");
				break;
			}
		} 


		last
	}

	


	#[test]
	fn test_part_1() {
		//let p1 = part1(&initial, &"HOH");
		assert_eq!(part1_2(4,2), 32451966);
		assert_eq!(part1_2(6,5), 1534922);
		//assert_eq!(p2, 689);
	}
	
}
