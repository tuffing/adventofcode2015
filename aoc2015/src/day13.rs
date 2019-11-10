pub mod solution {
	use std::cell::RefCell;

	pub struct Location {
		distances: RefCell<Vec<i64>>
	}

	impl Location {
		pub fn new()->Location {
			Location {distances: RefCell::new(Vec::new()) }
		}
		pub fn add(&self, value: i64){  
			self.distances.borrow_mut().push(value);
		}
	}

	pub fn run(person_count: usize) -> (i64, i64) {
		let _input = match crate::loadinput::file::load_input_multi_line("../../inputs/day13/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				return (0,0);
			},
		};

		part1_2(&_input, person_count)
	}


	pub fn generate(distances: &Vec<Location>) -> i64 {
		//heaps algorithm to get every permutation
		//https://en.wikipedia.org/wiki/Heap%27s_algorithm
		let mut c =  Vec::new();
		let mut elements = Vec::new();

		for x in 0..distances.len() {
			c.push(0);
			elements.push(x);
		}

		let mut shortest = sum_track(&elements, &distances);
		let mut longest = shortest;

		let mut i = 0;
		while i < elements.len() {
			if c[i] < i {
				if i % 2 == 0 {
					elements.swap(0,i);
				}
				else {
					elements.swap(c[i], i);
				}

				let sum = sum_track(&elements, &distances);
				
				if sum < shortest {
					shortest = sum;
				}

				if sum > longest {
					longest = sum;
				}

				c[i] += 1;

				i = 0;
			}
			else {
				c[i] = 0;
				i += 1;
			}
		}

		longest
	}

	pub fn sum_track(order: &Vec<(usize)>, distances: &Vec<Location>) -> i64 {
		let mut current = order[order.len()-1];
		let mut sum = 0;

		for x in  0..order.len() {
			sum += distances[current].distances.borrow()[order[x]];
			sum += distances[order[x]].distances.borrow()[current];

			current = order[x];
		}

		sum
	}

	pub fn part1_2(input: &Vec<(String)>, person_count: usize) -> (i64,i64) {
		//this is just being solved using an identical solution to day 9 just with a slightly different
		//parse and sum function. rust is fast enough to brute force this very quickly.
		//I suspect there might be an algorithm intended for this.

		//the parsing algorithm here is simply going to build a 2d array of every distance pair
		let mut distances = Vec::new();
		let mut from_index = 0;
		let mut to_index = 1;
		let mut current = "";

		for _ in 0..person_count {
			let row = Location::new();
			for _ in 0..person_count {
				row.add(0);
			}
			distances.push(row);
		}
		//Alice would gain 54 happiness units by sitting next to Bob.
		for s in input {
			let parts: Vec<&str> = s.split(' ').collect();
			let from = parts[0];

			if current == "" {
				current = from;
			}
			else if current != from  {
				current = from;				
				from_index += 1;
				to_index = 0;
			}

			let mut dist = parts[3].parse::<i64>().unwrap();

			if parts[2] == "lose" {
				dist = dist * -1;
			}
			distances[from_index].distances.borrow_mut()[to_index] = dist;

			to_index += 1;

			if from_index == to_index {
				to_index += 1;
			}
		}

		//(0,0)
		let p1 = generate(&distances);

		//for part 2 need to insert a row
		for x in distances.iter() {
			x.distances.borrow_mut().push(0);
		}

		let row = Location::new();
		for _ in 0..person_count +1 {
			row.add(0);
		}
		distances.push(row);

		let p2 = generate(&distances);

		(p1, p2)
	}



	#[test]
	fn test_part_1_2() {

		let (p1,_) = part1_2(&vec![
			"Alice would gain 54 happiness units by sitting next to Bob.".to_string(),
			"Alice would lose 79 happiness units by sitting next to Carol.".to_string(),
			"Alice would lose 2 happiness units by sitting next to David.".to_string(),
			"Bob would gain 83 happiness units by sitting next to Alice.".to_string(),
			"Bob would lose 7 happiness units by sitting next to Carol.".to_string(),
			"Bob would lose 63 happiness units by sitting next to David.".to_string(),
			"Carol would lose 62 happiness units by sitting next to Alice.".to_string(),
			"Carol would gain 60 happiness units by sitting next to Bob.".to_string(),
			"Carol would gain 55 happiness units by sitting next to David.".to_string(),
			"David would gain 46 happiness units by sitting next to Alice.".to_string(),
			"David would lose 7 happiness units by sitting next to Bob.".to_string(),
			"David would gain 41 happiness units by sitting next to Carol.".to_string()
		], 4);

		assert_eq!(p1, 330);
	}

}

