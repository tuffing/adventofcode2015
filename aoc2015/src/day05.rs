pub mod solution {
	extern crate fancy_regex;
	use fancy_regex::*;
	
		pub fn run() -> (usize, usize) {	
		let input = match crate::loadinput::file::load_input_multi_line("../../inputs/day05/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		part_1_and_2(input)
	}

	///
	///	# Part 1:
	///
	///	a: It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
	///	b: It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
	///	c: It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
	///
	///	# Part 2
  ///  a. It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy)
  ///		 or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
  ///  b. It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe),
  ///		 or even aaa.
	///
	pub fn part_1_and_2(input: Vec<(String)>) -> (usize, usize) {
		let part1_a = Regex::new(r"([aeiou]\w*){3}").unwrap();
		let part1_b = Regex::new(r"(\w)\1").unwrap();
		let part1_c = Regex::new(r"(ab)|(cd)|(pq)|(xy)").unwrap();

		let part2_a = Regex::new(r"(\w\w)\w*\1").unwrap();
		let part2_b = Regex::new(r"(\w)\w\1").unwrap();

		let mut part1_count = 0;
		let mut part2_count = 0;

		for line in input {
			if part1_a.is_match(&line).unwrap() && part1_b.is_match(&line).unwrap() && !part1_c.is_match(&line).unwrap() {
				part1_count += 1;
			}

			if part2_a.is_match(&line).unwrap() && part2_b.is_match(&line).unwrap() {
				part2_count += 1;
			}
		}

		(part1_count, part2_count)
	}



	#[test]
	fn test_part_1() {
		let (part1, _) = part_1_and_2(vec!["ugknbfddgicrmopn".to_string(),
			"aaa".to_string(),
			"jchzalrnumimnmhp".to_string(),
			"haegwjzuvuyypxyu".to_string(),
			"dvszwmarrgswjxmb".to_string()
			]
		);

		assert_eq!(part1, 2);
	}
	
	#[test]
	fn test_part_2() {
		let (_, part2) = part_1_and_2(vec!["qjhvhtzxzqqjkmpb".to_string(),
			"xxyxx".to_string(),
			"uurcxstgmygtbstg".to_string(),
			"ieodomkazucvgmuy".to_string(),
			]
		);

		assert_eq!(part2, 2);
	}

}

