pub mod solution {
	pub fn run() -> (String, String) {
		let part1 = part1_2("hepxcrrq".to_string());
		let part2 = part1_2(part1.clone());

		(part1, part2)
	}

	pub fn part1_2(input: String) -> String {
		let mut registry: [usize; 8] = [0; 8];

		//set up the registry
		let mut input_iter = input.chars();
		for x in 0..input.len() {
			let index = "abcdefghijklmnopqrstuvwxyz".find(input_iter.next().unwrap());
			registry[x] = index.unwrap();
		}


		for _ in 0..6436343 {
			increment(&mut registry, 7);

			if validate(&registry) {
				break;
			}
		}

		let answer = registry_to_string(&registry);

		answer
	}

	pub fn registry_to_string(registry: &[usize; 8]) -> String {
		let mut result = String::new();
		let abc = "abcdefghijklmnopqrstuvwxyz";

		for x in registry.iter() {
			result.push(abc.chars().nth(*x).unwrap());
		}

		result
	}

	pub fn increment(mut registry: &mut [usize; 8], index: usize) {
		registry[index] += 1;

		if registry[index] > 25 {
			registry[index] = 0;
		}

		//o,l,i respectively
		if [14, 11, 8].contains(&registry[index]) {
			registry[index] += 1;
		}

		if registry[index] == 0 && index > 0 {
			increment(&mut registry, index - 1);
		}
	}


	/*
	Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
	Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
	Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
	*/
	pub fn validate(registry: &[usize; 8]) -> bool {
		let mut previous: i32 = 0;

		let mut three_in_a_row_score = 2;
		let mut three_in_a_row = false;

		let mut pair_index = 10;
		let mut pair1 = false;
		let mut pair2 = false;

		for (i, x) in registry.iter().enumerate() {
			let curr = *x as i32;

			//3 in a row
			if previous - curr != -1 {
				//reset the score
				three_in_a_row_score = 2;
			}
			else {
				three_in_a_row_score -= 1;
				if three_in_a_row_score == 0 {
					three_in_a_row = true;
				}
			}

			//pair check
			if previous - curr == 0 && i != 0 {
				if pair1 == false {
					pair1 = true;
				}
				else if i-pair_index != 1 { //prevent overlap
					pair2 = true;
				}

				pair_index = i;
			}

			previous = curr;
		}


		three_in_a_row && pair1 && pair2
	}

	#[test]
	fn test_part_1_2() {

		let p1 = part1_2("abcdefgh".to_string());

		assert_eq!(p1, "abcdffaa".to_string());
	}

}

