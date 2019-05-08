pub mod solution {
	extern crate crypto;

	use crypto::md5::Md5;
	use crypto::digest::Digest;
	
	pub fn run(input: &str) -> (usize, usize) {	
		part1_2(&input)
	}

	/// THis is just brute forcing 0 onwards to find the answers
	/// as a result it is going to be very slow. cant think of a 
	/// better solution this time.
	///I have a suspicion that crypto doesn't have the best md5 algorithm.
	fn part1_2(input: &str) -> (usize, usize) {
		let mut count = 0;
		let mut part1 = 0;
		let mut part2 = 0;
		let mut hasher = Md5::new();

		loop {
			let num = format!("{}{}", input, count);
			hasher.input_str(&num);
			let hash = hasher.result_str();

			if part1 == 0 && hash.starts_with("00000") {
				part1 = count;
			}

			if part2 == 0 && hash.starts_with("000000") {
				part2 = count;
			}

			count += 1;

			if (part1 > 0 && part2 > 0) || count > 10000000 {
				return (part1, part2);
			}

			hasher.reset();
		}
	}

	#[test] 
	fn test_part_1() {
		let mut part1_sol = part1_2("abcdef");
		assert_eq!(part1_sol.0, 609043);

		part1_sol = part1_2("pqrstuv");
		assert_eq!(part1_sol.0, 1048970);
	}

}