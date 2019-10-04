pub mod solution {

	pub fn run() -> (usize, usize) {
		let (part1, part2) = part1_2("1113222113".to_string(), 40, 50);

		(part1, part2)
	}

	pub fn part1_2(input: String, p1_iterations: usize, p2_iterations: usize) -> (usize,usize) {
		let mut curr = input;
		let mut p1_curr = 0;
		for x in 0..p2_iterations {
			let mut new_iter = String::new();
			let mut num = curr.chars().next().unwrap();
			curr.push_str(".");
			let mut count = 0;

			for c in curr.chars() {
				if c == num {
					count += 1;
				}
				else {
					new_iter.push_str(&count.to_string());
					new_iter.push(num);
					count = 1;
					num = c;
				}
			}

			curr = new_iter;
			
			if x == p1_iterations - 1 {
				p1_curr = curr.len();
			}

		}


		(p1_curr,curr.len())
	}


	#[test]
	fn test_part_1_2() {

		let (p1,p2) = part1_2("1".to_string(), 5, 6);

		assert_eq!(p1, 6);
		//assert_eq!(p2, 982);
	}

}

