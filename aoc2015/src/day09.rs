pub mod solution {
	use std::cell::RefCell;

	pub struct Location {
		distances: RefCell<Vec<usize>>
	}

	impl Location {
		pub fn new()->Location {
			Location {distances: RefCell::new(Vec::new()) }
		}
		pub fn add(&self, value: usize){  
			self.distances.borrow_mut().push(value);
		}
	}

	pub fn run(city_count: usize) -> (usize, usize) {
		let _input = match crate::loadinput::file::load_input_multi_line("../../inputs/day09/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		part1_2(&_input, city_count)
	}


	pub fn generate(distances: &Vec<Location>) -> (usize,usize) {
		//heaps algorithm to get every permutation
		//https://en.wikipedia.org/wiki/Heap%27s_algorithm
		let mut c =  Vec::new();
		//let n = 4;
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

		(shortest, longest)
	}

	pub fn sum_track(order: &Vec<(usize)>, distances: &Vec<Location>) -> usize {
		let mut current = order[0];
		let mut sum = 0;

		for x in  1..order.len() {
			sum += distances[current].distances.borrow()[order[x]];
			current = order[x];
		}

		sum
	}

	pub fn part1_2(input: &Vec<(String)>, city_count: usize) -> (usize,usize) {
		//the parsing algorithm here is simply going to build a 2d array of every distance pair
		let mut distances = Vec::new();
		let mut from_index = 0;
		let mut to_index = 1;
		let mut current = "";

		for _ in 0..city_count {
			let row = Location::new();
			for _ in 0..city_count {
				row.add(0);
			}
			distances.push(row);
		}
		
		for s in input {
			let parts: Vec<&str> = s.split(' ').collect();
			let from = parts[0];

			if current == "" {
				current = from;
			}
			else if current != from  {
				current = from;
				from_index += 1;
				to_index = from_index + 1;
			}

			let dist = parts[4].parse::<usize>().unwrap();

			distances[from_index].distances.borrow_mut()[to_index] = dist;
			distances[to_index].distances.borrow_mut()[from_index] = dist;

			to_index += 1;
		}


		generate(&distances)
	}



	#[test]
	fn test_part_1_2() {

		let (p1,p2) = part1_2(&vec![
			"London to Dublin = 464".to_string(),
			"London to Belfast = 518".to_string(),
			"Dublin to Belfast = 141".to_string()
		], 3);

		assert_eq!(p1, 605);
		assert_eq!(p2, 982);
	}

}

