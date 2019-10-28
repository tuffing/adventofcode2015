pub mod solution {
	use serde_json::{Value};
	use regex::Regex;

	pub fn run() -> (i64, i64) {	
		let input = match crate::loadinput::file::load_input_single_line("../../inputs/day12/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		(part1(&input),part2(input))
	}

	pub fn part1(input: &String) -> i64 {
		//part one is easily solvable with regex.
		//get all the numbers out of the text then add them together. 
		//i64 as there are negatives
		let mut total = 0;
		let re = Regex::new(r"-?\d+").unwrap();
		for cap in re.captures_iter(input) {
			total += cap.get(0).map_or(0, |m| m.as_str().parse::<i64>().unwrap());
		}

		total
	}

	pub fn part2(input: String) -> i64 {
		//this time we must ignore values who share a group with 'red'.
		//so we need to parse it into an object. in this case it happens to be json, so existing libraries can do that.
		//we cycle over each level of the json, if it has a property of 'red' we stop and try other children.
		//This could be very easily be done via recursion. Opted for iterative mostly out of habit from python.

		let v: Value = serde_json::from_str(&input).unwrap();

		let mut values = Vec::new();
		let mut total: i64 = 0;
		values.push(&v); 

		while values.len() > 0 {
			let v = values.pop().unwrap();

			let mut new_arr: Vec<&serde_json::value::Value> = Vec::new();
			let mut value_so_far = 0;

			
			if v.is_array() {
				let arr = v.as_array().unwrap();

				for x in arr.iter() {
					if x.is_number() {
						value_so_far += x.as_i64().unwrap();
					}
					else if x.is_array() || x.is_object() {
						new_arr.push(x);
					}
				}
			}

			//even though an object in this case shares the same functions we need as for a Vec, 
			//we need to run them sperately again here. there doesn't seem to be a useful shared ancestor 
			//for the object here and a vec to grasp onto. 
			if v.is_object() {
				let obj = v.as_object().unwrap();

				for (_,x) in obj.iter() {
					if x.is_string() && x.as_str().unwrap() == "red" {
						value_so_far = 0;
						new_arr.clear();
						break;
					}
					else if x.is_number() {
						value_so_far += x.as_i64().unwrap();

					}
					else if x.is_array() || x.is_object() {
						new_arr.push(x);
					}
				}
			}

			total += value_so_far;
			values.append(&mut new_arr);
		}

		total
	}

	#[test]
	fn test_part_1_2() {

		let p2a = part2("[1,2,3]".to_string());
		let p2b = part2("[1,{\"c\":\"red\",\"b\":2},3]".to_string());
		let p2c = part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".to_string());
		let p2d = part2("[1,\"red\",5]".to_string());

		assert_eq!(p2a, 6);
		assert_eq!(p2b, 4);
		assert_eq!(p2c, 0);
		assert_eq!(p2d, 6);
	}

}

