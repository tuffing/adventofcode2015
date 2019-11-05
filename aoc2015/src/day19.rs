pub mod solution {
	extern crate rand;

	use std::collections::HashSet;
	use regex::Regex;
	use rand::Rng;


	pub fn run() -> (usize, usize) {
		let _input = match crate::loadinput::file::load_input_multi_line("../../inputs/day19/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				return (0,0);
			},
		};

		let seed = "ORnPBPMgArCaCaCaSiThCaCaSiThCaCaPBSiRnFArRnFArCaCaSiThCaCaSiThCaCaCaCaCaCaSiRnFYFArSiRnMgArCaSiRnPTiTiBFYPBFArSiRnCaSiRnTiRnFArSiAlArPTiBPTiRnCaSiAlArCaPTiTiBPMgYFArPTiRnFArSiRnCaCaFArRnCaFArCaSiRnSiRnMgArFYCaSiRnMgArCaCaSiThPRnFArPBCaSiRnMgArCaCaSiThCaSiRnTiMgArFArSiThSiThCaCaSiRnMgArCaCaSiRnFArTiBPTiRnCaSiAlArCaPTiRnFArPBPBCaCaSiThCaPBSiThPRnFArSiThCaSiThCaSiThCaPTiBSiRnFYFArCaCaPRnFArPBCaCaPBSiRnTiRnFArCaPRnFArSiRnCaCaCaSiThCaRnCaFArYCaSiRnFArBCaCaCaSiThFArPBFArCaSiRnFArRnCaCaCaFArSiRnFArTiRnPMgArF";

		(part1(&_input, &seed),part2(&_input, &seed))
	}


	pub fn part1(input: &Vec<(String)>, seed: &str) -> usize {
		//mapping: one vec for the keys one for the transformation.
		//seperate as the keys are repeatable - aka not approriate for a map
		let mut keys = Vec::new();
		let mut values = Vec::new();
		let mut results = HashSet::new();

		for s in input {
			let parts: Vec<&str> = s.split(" => ").collect();

			keys.push(parts[0]);
			values.push(parts[1]);
		}


		for i in 0..keys.len() {
			for mat in Regex::new(keys[i]).unwrap().find_iter(&seed) {
				let mut string = seed[..mat.start()].to_string();
				string.push_str(values[i]);
				string.push_str(&seed[mat.end()..]);

				results.insert(string);
			}
		}



		results.len()
	}

	pub fn part2(input: &Vec<(String)>, seed: &str) -> usize {
		//mapping this time i've changed it to tuples
		//mostly to make it easier to change the indexes
		//and also because i remembered rust has tuples

		let mut values = Vec::new();

		for s in input {
			let parts: Vec<&str> = s.split(" => ").collect();
			values.push((parts[0],parts[1]));
		}

		let mut total = 0;
		//escape is just a way to kill the loop and prevent an infinite loop.
		let mut escape = 0;
		let mut current = seed.to_string();

		while escape < 1500 && current != "e" {
			let prev = current.clone();
			let mut new = "".to_string();

			for i in (0..values.len()).rev() {
				//if the string replace finds nothing then we don't count it
				//only do the first match or we won't know the number of matches
				//Interesting note here: string replace is much much faster than regex replace
				new = current.replacen(values[i].1, values[i].0,1).to_string();

				if (new != current) {
					total += 1;
					current = new.to_string();
					break;
				}

				if current == "e" {
					break;
				}
			}

			//if prev = current then the current order isn't working out.
			//shuffle it up and try again. 
			//below is a basic shuffle algorithm that should do the trick
			if prev == current {
				let mut new_index = Vec::new();
				new_index.push(values.pop().unwrap());
				while values.len() > 0 {
					let num = rand::thread_rng().gen_range(0, new_index.len());
					new_index.insert(num,values.pop().unwrap());
				}
				values = new_index;
				current = seed.to_string();
				total = 0;
			}

			escape+=1;
		}

		total
	}

	
	#[test]
	fn test_part_1() {
		let initial = &vec![
			"H => HO".to_string(),
			"H => OH".to_string(),
			"O => HH".to_string()
		];


		let p1 = part1(&initial, &"HOH");

		assert_eq!(p1, 4);
		//assert_eq!(p2, 689);
	}

	#[test]
	fn test_part_2() {
		let initial = &vec![
			"e => H".to_string(),
			"e => O".to_string(),
			"H => HO".to_string(),
			"H => OH".to_string(),
			"O => HH".to_string()
		];


		let p2a = part2(&initial, &"HOH");
		let p2b = part2(&initial, &"HOHOHO");

		assert_eq!(p2a, 3);
		assert_eq!(p2b, 6);
		//assert_eq!(p2, 689);
	}

}

