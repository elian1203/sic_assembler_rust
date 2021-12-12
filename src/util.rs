use std::fs::File;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
	let file_result = File::open(filename);
	if file_result.is_err() {
		println!("Could not open file!");
		exit(1);
	} else {
		let file = file_result.unwrap();
		let reader = io::BufReader::new(file);
		return Ok(reader.lines());
	}
}

pub fn write_lines<P: AsRef<Path>>(filename: P, lines: Vec<String>) {
	let joined = lines.join("\n");
	let result = fs::write(filename, joined);
	if result.is_err() {
		println!("Could not write to file! Check folder permissions.");
	}
}

pub fn parse_str_i32_or_error(str: Option<&str>, base: u32, error_message: String) -> i32 {
	if str.is_none() {
		println!("{}", error_message);
		exit(1);
	}
	let parsed = i32::from_str_radix(str.unwrap(), base);
	if parsed.is_err() {
		println!("{}", error_message);
		exit(1);
	}
	return parsed.unwrap();
}

pub fn parse_string_i32_or_error(str: Option<&String>, base: u32, error_message: String) -> i32 {
	if str.is_none() {
		println!("{}", error_message);
		exit(1);
	}
	let parsed = i32::from_str_radix(str.unwrap(), base);
	if parsed.is_err() {
		println!("{}", error_message);
		exit(1);
	}
	return parsed.unwrap();
}
