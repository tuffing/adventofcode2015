pub mod solution {
	pub struct Result {
		smallest_count: usize,
		lowest_qe: usize
	}

	pub fn run() -> (usize, usize) {
		let input = &vec![113,109,107,103,101,97,89,83,79,73,71,67,59,53,47,43,41,37,31,29,23,19,17,13,11,5,3,1];

		(part1_2(&input, 3), part1_2(&input, 4))
	}


	pub fn part1_2(values: &Vec<usize>, split: usize) -> (usize) {
		let mut sum = 0;
		for v in values.iter() {
			sum += v;
		}
		let goal = sum / split;

		let mut result = Result { smallest_count: 6, lowest_qe: usize::max_value() };
		for i in 0..values.len() {
			find(&mut result, goal, 1, 0, 0, i, &values);
		}

		result.lowest_qe
	}

	pub fn find(result: &mut Result, goal: usize, qe: usize, weight: usize, count: usize, next_index: usize, options: &Vec<usize>)  {
		//just recursively trying every option until we get there.
		//lots of ways to kill the recursion so to try and avoid trying duplicates in different orders 
		//or path ways that will never work. Some will still get through of course.
		let new_count = count + 1;
		let new_qe = qe * options[next_index];

		let new_weight = weight + options[next_index];

		if new_weight == goal && new_count <= result.smallest_count {
			if new_count == result.smallest_count &&  new_qe < result.lowest_qe {
				result.lowest_qe = new_qe;
			}
			else if new_count < result.smallest_count {
				result.smallest_count = new_count;
				result.lowest_qe = new_qe;
			}

			return;
		}

		if new_weight >= goal || new_count >= result.smallest_count || new_qe >= result.lowest_qe {
			return;
		}

		let mut new_options = options.to_vec();
		new_options.remove(next_index);

		let remaining = goal as i64 - new_weight as i64 ;
		for i in 0..new_options.len() {
			if new_count + 3 >= result.smallest_count &&  remaining - (new_options[i] * 3) as i64 > 0 {
				//options are pre sorted descending. so if this doesn't fix, none will.
				//this is a nice simple way to kill the recursion early. If none of the current item can fit x times into
				//the remaining weight then it's impossible to happen at all with this item or any after 
				break;
			}
			find(result, goal, new_qe, new_weight, new_count, i, &new_options);
		}
	}


	#[test]
	fn test_part_1() {
		let values = &vec![11,10,9,8,7,5,4,3,2,1];


		//let p1 = part1(&initial, &"HOH");
		let p1 = part1_2(&values, 3);
		let p2 = part1_2(&values, 4);

		assert_eq!(p1, 99);
		assert_eq!(p2, 44);
		//assert_eq!(p2, 689);
	}
	
}

