pub mod solution {
	use std::collections::HashMap;

	pub fn run() -> (usize, usize) {
		let input = match crate::loadinput::file::load_input_multi_line("../../inputs/day23/input.txt") {
			Ok(val) => val,
			Err(_) => {
				println!("NO FILE");
				return (0,0);
			},
		};

		(part1_2(&input, 0, "b".to_string()),part1_2(&input, 1, "b".to_string()))
	}


	pub fn part1_2(input: &Vec<(String)>, reg_a: usize, answer_reg: String) -> (usize) {
		let mut functions: HashMap<&str, fn(&mut HashMap<std::string::String, usize>, &str, i64) -> i64>  = HashMap::new();
		functions.insert(&"hlf", hlf);
		functions.insert(&"tpl", tpl);
		functions.insert(&"inc", inc);
		functions.insert(&"jmp", jmp);
		functions.insert(&"jie", jie);
		functions.insert(&"jio", jio);

		let mut regs = HashMap::new();
		regs.insert("a".to_string(), reg_a);
		regs.insert("b".to_string(), 0);
		
		let mut instructions = Vec::new();
		for s in input.iter() {
			//all instructions are 3 char
			//let mut working = s.clone();
			let (instr, args) = s.split_at(3);
			instructions.push((instr, args.trim_start().to_string()));
		}


		//run the simulation
		let mut index: i64 = 0;
		while index >= 0 && index < instructions.len() as i64 {
			let usize_index = index as usize;
			//println!("index {:?}, args {:?}", index, &instructions[usize_index]);
			index = functions[instructions[usize_index].0](&mut regs, &instructions[usize_index].1, index);
			//println!("result {:?}", regs);
		}


		regs[&answer_reg]
	}

	//hlf r sets register r to half its current value, then continues with the next instruction.
	fn hlf(regs: &mut HashMap<String, usize>, args: &str, instruction_index: i64) -> i64 {
		if regs[args] > 0 {
			let val = regs[args];
			regs.insert(args.to_string(), val / 2);
		}

		instruction_index + 1
	}

	//tpl r sets register r to triple its current value, then continues with the next instruction.
	fn tpl(regs: &mut HashMap<String, usize>, args: &str, instruction_index: i64) -> i64  {
		let val = regs[args];
		regs.insert(args.to_string(), val * 3);
		
		instruction_index + 1
	}


	//inc r increments register r, adding 1 to it, then continues with the next instruction.
	fn inc(regs: &mut HashMap<String, usize>, args: &str, instruction_index: i64) -> i64 {
		let val = regs[args];
		regs.insert(args.to_string(), val + 1);

		instruction_index + 1
	}


	fn jmp(_regs: &mut HashMap<String, usize>, args: &str, instruction_index: i64)  -> i64 {
		//jmp offset is a jump; it continues with the instruction offset away relative to itself.
		//println!("{:?}", ("-123".to_string()).parse::<i64>());
		instruction_index + args.parse::<i64>().unwrap()
	}

	//jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
	fn jie(regs: &mut HashMap<String, usize>, args: &str, instruction_index: i64)  -> i64 {
		let parts: Vec<&str> = args.split(", ").collect();
		//	println!("HERE {:?}", regs[&parts[0].to_string()]);

		if regs[&parts[0].to_string()] % 2 == 0 {
			return instruction_index + parts[1].parse::<i64>().unwrap()
		}

		instruction_index + 1
	}

	//jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
	fn jio(regs: &mut HashMap<String, usize>, args: &str, instruction_index: i64)  -> i64 {
		let parts: Vec<&str> = args.split(", ").collect();

		if regs[&parts[0].to_string()] == 1 {
			return instruction_index + parts[1].parse::<i64>().unwrap()
		}

		instruction_index + 1
	}

	#[test]
	fn test_part_1() {
		let initial = &vec![
			"inc a".to_string(),
			"jio a, +2".to_string(),
			"tpl a".to_string(),
			"inc a".to_string()
		];


		//let p1 = part1(&initial, &"HOH");
		let p1 = part1_2(&initial, 0, "a".to_string());

		assert_eq!(p1, 2);
		//assert_eq!(p2, 689);
	}
	
}

