pub mod solution {
	use std::collections::HashMap;

	pub fn run() -> (u16, u16) {	
		let mut input = match crate::loadinput::file::load_input_multi_line("../../inputs/day07/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		let part1 = solution(&input, "a");
		input.insert(0,  format!("{} -> b", part1));
		let part2 = solution(&input, "a");

		(part1, part2)
	}

	///
    /// 123 -> x means that the signal 123 is provided to wire x.
    /// x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
    /// p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
    /// NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.
	///
	pub fn solution(input: &Vec<(String)>, key: &str) -> u16 {
		let mut wires: HashMap<String, u16> = HashMap::new();
		let length = input.len();

		for _i in 1..length {
			run_process(&input, &mut wires);
			if wires.len() >= length {
				break;
			}
		}
		*wires.get(key).unwrap()
	}

	fn get_value_from_parts(index: usize, parts: &Vec<&str>, wires: &HashMap<String, u16>) -> u16 {
		let num = parts[index].parse::<u16>();
		let x;
		if num.is_ok() {
			x = num.unwrap();
		}
		else {
			x = *wires.get(&parts[index].to_string()).unwrap();
		}

		return x;
	}

	fn index_exists_or_is_num(index: usize, parts: &Vec<&str>, wires: &HashMap<String, u16>) -> bool {
		let num = parts[index].parse::<u16>();

		return num.is_ok() || wires.contains_key(&parts[index].to_string());
	}

	fn run_process(input: &Vec<(String)>, wires: &mut HashMap<String, u16>) {			
		for s in input {
			let parts: Vec<&str> = s.split(' ').collect();
			if wires.contains_key(parts[parts.len() - 1]) {
				continue;
			} 

			if parts.len() == 3 && index_exists_or_is_num(0, &parts, &wires) { //if it's len 3 then it has to be 123 -> x
				let x = get_value_from_parts(0, &parts, &wires);

				wires.insert(parts[2].to_string(), x);
			}
			else if parts.len() == 4 && index_exists_or_is_num(1, &parts, &wires) { //len 4 must be NOT e -> f
				wires.insert(parts[3].to_string(), !wires.get(&parts[1].to_string()).unwrap());
			}
			else if parts.len() >= 4 && index_exists_or_is_num(0, &parts, &wires) && index_exists_or_is_num(2, &parts, &wires) {
				let x = get_value_from_parts(0, &parts, &wires);			
				let y = get_value_from_parts(2, &parts, &wires);

				if parts[1] == "AND" { //x AND y -> d
					wires.insert(parts[4].to_string(),  x & y);
				}
				else if parts[1] == "OR" { //x OR y -> d
					wires.insert(parts[4].to_string(), x | y);
				}
				else if parts[1] == "LSHIFT" { //x LSHIFT 2 -> f
					wires.insert(parts[4].to_string(), x << y);
				}
				else if parts[1] == "RSHIFT" { //x LSHIFT 2 -> f
					wires.insert(parts[4].to_string(), x >> y);
				}
			}
		}
	}

	#[test]
	fn test_part_1() {
		let part1 = solution(&vec![
				"x LSHIFT 2 -> f".to_string(),
				"y RSHIFT 2 -> g".to_string(),
				"NOT x -> h".to_string(),
				"NOT y -> i".to_string(),
				"x OR y -> e".to_string(),
				"x AND y -> d".to_string(),
				"1 AND y -> j".to_string(),
				"y -> k".to_string(),
				"123 -> x".to_string(),
				"456 -> y".to_string(),
			], "f"
		);

		assert_eq!(part1, 492);
	}

}

