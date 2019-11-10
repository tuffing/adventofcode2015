pub mod solution {
	use regex::Regex;

	pub fn run() -> (usize, usize) {	
		let _input = match crate::loadinput::file::load_input_multi_line("../../inputs/day08/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		part1_2(&_input)
	}

	pub fn part1_2(input: &Vec<(String)>) -> (usize,usize) {
		//both of these problems don't need us to actually encode he string per the rules
		//we just need to strip out / add characters and do some math

		//part 1: the total number of characters to represent the newly encoded strings
		// minus the number of characters of code in each original string literal
		let start_qoutes = Regex::new(r#"^"|"$|\\"#).unwrap();
		let hex = Regex::new(r#"\\{2}|\\x[0-9a-f]{2}"#).unwrap();
		//Your task is to find the total number of characters to represent the newly encoded strings 
		//minus the number of characters of code in each original string literal.
		let double_chars = Regex::new(r#"\\|""#).unwrap();

		let mut original_length = 0;
		let mut part_1_alter = 0;
		let mut part_2_alter = 0;

		for s in input {
			original_length += s.len();
			//what we decode / encode into is arbitrary, just as long as the new string is the right length

			let decoded = hex.replace_all(&s, "X");
			let decoded = start_qoutes.replace_all(&decoded, "");

			let encoded = double_chars.replace_all(&s, "12"); 

			part_1_alter += decoded.len();
			//our encoded string needs to be wrapped in new quotes.. so two more characters
			part_2_alter += encoded.len() + 2;
		}

		(original_length - part_1_alter, part_2_alter - original_length)
	}

	#[test]
	fn test_part_1_2() {
		let (p1,p2) = part1_2(&vec![
				"\"\"".to_string(),
				"\"abc\"".to_string(),
				"\"aaa\\\"aaa\"".to_string(),
				"\"\\x27\"".to_string()
			]);

		assert_eq!(p1, 12);
		assert_eq!(p2, 19);
	}

}

