pub mod solution {
	pub fn run() -> (usize, usize) {

		part1_2()
	}


	pub fn part1_2() -> (usize,usize) {
		let length = 787000;
		let mut houses_p1 = vec![0; length+1];
		let mut houses_p2 = vec![0; length+1];

		let goal = 33100000;

		let mut lowest_index_p1 = length;
		let mut lowest_index_p2 = length;
		let p2_cap = 50;

		for i in 1..length {
			let mut count = 0;
			for p in (i..length).step_by(i) {
				houses_p1[p] += i * 10;

				if houses_p1[p] >= goal && i < lowest_index_p1 {
					lowest_index_p1 = i;
				}

				if count < p2_cap {
					houses_p2[p] += i * 11;

					if houses_p2[p] >= goal && i < lowest_index_p2 {
						lowest_index_p2 = i;
					}
				}
				count += 1;
			}
		}

		(lowest_index_p1, lowest_index_p2)
	}
	
}

