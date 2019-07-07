///Common functions go here. most likely just file loading and parsing.
pub mod file {
	use std::fs::File;
	use std::fs;
	use std::io::{BufRead, BufReader, Result};


	pub fn load_input_single_line(filename: &str) -> Result<(String)> {
		let file = File::open(filename)?;
		let mut test = String::new();

		for line in BufReader::new(file).lines() {
			test += &line?;
		}

		Ok(test)
	}

	pub fn load_input_multi_line(filename: &str) -> Result<(Vec<String>)> {
		let rows = fs::read_to_string(filename).expect("");
		Ok(rows.split('\n').map(|x| x.to_string()).collect())
	}
}