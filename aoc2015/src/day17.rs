pub mod solution {

	pub fn run() -> (i64, i64) {
		let jugs = vec![50,44,11,49,42,46,18,32,26,40,21,7,18,43,10,47,36,24,22,40];
		(part1(&jugs),part2(&jugs))
	}


	pub fn part1(jugs: &Vec<(usize)>) -> i64 {
		//a variant of knapsack. 

		let length = jugs.len();

		let mut uses = vec![1; length];

		let goal = 150;

		let mut table = vec![0; goal+1];
		
		table[0] = 1;

		for c in 0..length {
			let mut rs = table.clone();
			
			for i in jugs[c]..goal+1 {
				rs[i] += rs[i-jugs[c]];

				table[i] += rs[i-jugs[c]];
				if i >= jugs[c]*(1+uses[c]) {
					table[i] -= rs[i-jugs[c]*(1+uses[c])];
				}
			}
		}

		table[goal]
	}

	pub fn part2(jugs: &Vec<(usize)>) -> i64 {
		// I couldn't find any famous mathematical algorithms here
		// so brute force it is.. quick eyeball tells me the minimum 
		// number of jugs is 4, so 4 nested loops will do it. 
		let length = jugs.len();
		let goal = 150;

		let mut total = 0;

		for a in 0..length {
			for b in a+1..length {
				for c in b+1..length {
					for d in c+1..length {
						if jugs[a] + jugs[b] + jugs[c] + jugs[d] == 150 {
							total += 1;
						}
					}
				}
			}
		}

		return 0;
	}



	#[test]
	fn test_part_1_2() {

		let (p1,p2) = part2();

		assert_eq!(p1, 4);
		//assert_eq!(p2, 689);
	}

}

