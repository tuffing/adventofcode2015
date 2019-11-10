pub mod solution {

	pub fn run() -> (i64, i64) {
		part1_2()
	}


	pub fn part1_2() -> (i64,i64) {
		/* 
			broke my usual pattern for this one and didn't implement a parser
			I would need to implement a way to allow for arbitary nested for loops, which seems messy.

			I spent hours researching linear algorithms that could solve this puzzle with out brute forcing,
			then decided to bruteforce it anyway. I expected this to be a slow method, It solves instantly, feels ugly, but it's very fast..
		*/

		const CAP: i64 = 100;
		let mut max_p1 = 0;
		let mut max_p2 = 0;

		for sprinkles in 1..CAP+1 {
			for peanuts in 1..CAP - sprinkles {
				for frosting in 1..CAP - sprinkles - peanuts {
					let sugar = CAP - sprinkles - peanuts - frosting;
					let (total,calories) = calculate(sprinkles, peanuts, frosting, sugar);

					if total > max_p1 {
						max_p1 = total;
					}

					if total > max_p2 && calories == 500 {
						max_p2 = total;
					}
				}
			}
		}



		(max_p1,max_p2)
	}

	pub fn calculate (sprinkle_count: i64, peanut_count: i64, frosting_count: i64, sugar_count: i64) -> (i64, i64) {
		let sprinkle = [5,-1,0,0,5];
		let peanut = [-1,3,0,0,1];
		let frosting = [0,-1,4,0,6];
		let sugar = [-1,0,0,2,8];

		let capacity =  (sprinkle[0] * sprinkle_count) + (peanut[0] * peanut_count) + (frosting[0] * frosting_count) + (sugar[0] * sugar_count);
		let durability =  (sprinkle[1] * sprinkle_count) + (peanut[1] * peanut_count) + (frosting[1] * frosting_count) + (sugar[1] * sugar_count);
		let flavor =  (sprinkle[2] * sprinkle_count) + (peanut[2] * peanut_count) + (frosting[2] * frosting_count) + (sugar[2] * sugar_count);
		let texture =  (sprinkle[3] * sprinkle_count) + (peanut[3] * peanut_count) + (frosting[3] * frosting_count) + (sugar[3] * sugar_count);
		let calories =  (sprinkle[4] * sprinkle_count) + (peanut[4] * peanut_count) + (frosting[4] * frosting_count) + (sugar[4] * sugar_count);

		if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
			return (0,0);
		}

		(capacity * durability * flavor * texture, calories)
	}


}

