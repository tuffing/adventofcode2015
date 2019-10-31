pub mod solution {
	use regex::Regex;
	use std::fs;


	pub fn run() -> (String, String) {	
		let _input = fs::read_to_string("../../inputs/day16/input.txt").expect("");

		(part1(&_input), part2(&_input))
	}

	pub fn part1(input: &String) -> String {
		/*
		Most straight forward way is just to run regex on the entire thing and then find the match at the end.

		children: 3
		cats: 7
		samoyeds: 2
		pomeranians: 3
		akitas: 0
		vizslas: 0
		goldfish: 5
		trees: 3
		cars: 2
		perfumes: 1*/
		let replacement = Regex::new(r#"children: 3"#).unwrap().replace_all(input, "M");
		let replacement = Regex::new(r#"cats: 7"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"samoyeds: 2"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"pomeranians: 3"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"akitas: 0"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"vizslas: 0"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"goldfish: 5"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"trees: 3"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"cars: 2"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"perfumes: 1"#).unwrap().replace_all(&replacement, "M");
		
		let winner = Regex::new(r#"(Sue \d+): M, M, M"#).unwrap();
		let caps = winner.captures(&replacement).unwrap();
		
		caps.get(1).unwrap().as_str().to_string()
	}

	pub fn part2(input: &String) -> String {
		/*
		Same as part 2 but new rules, just needs some minor tweaks to the regex:
		cats: 7 > than
		pomeranians: 3 < than
		goldfish: 5 < than
		trees: 3 > than*/
		let replacement = Regex::new(r#"children: 3"#).unwrap().replace_all(input, "M");

		let replacement = Regex::new(r#"cats: ([89]|10)"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"samoyeds: 2"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"pomeranians: [012]"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"akitas: 0"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"vizslas: 0"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"goldfish: [0-4]"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"trees: ([4-9]|10)"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"cars: 2"#).unwrap().replace_all(&replacement, "M");
		let replacement = Regex::new(r#"perfumes: 1"#).unwrap().replace_all(&replacement, "M");
		
		let winner = Regex::new(r#"(Sue \d+): M, M, M"#).unwrap();
		let caps = winner.captures(&replacement).unwrap();
		
		caps.get(1).unwrap().as_str().to_string()
	}


}

