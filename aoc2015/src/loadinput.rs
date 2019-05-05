///Common functions go here. most likely just file loading and parsing.
pub mod file {
	use std::fs::File;
	use std::io::{BufRead, BufReader, Result};


	pub fn load_input_single_line(file_name: &str) -> Result<(String)> {
		let file = File::open(file_name)?;
		let mut test = String::new();

		for line in BufReader::new(file).lines() {
			test += &line?;
		}

		Ok(test)
	}
}