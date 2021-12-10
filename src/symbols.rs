use std::cell::Cell;
use std::env::current_exe;
use std::fmt::Display;
use std::process::exit;
use std::string::String;

use crate::util;
use crate::instructions;
use crate::instructions::{is_directive, is_instruction};

struct Symbol {
	name: String,
	memory_location: i32,
}

pub struct SymbolTable {
	symbols: Vec<Symbol>,
	memory_locations: Vec<i32>,
	starting_memory_location: Cell<i32>,
	first_instruction: Cell<i32>,
	total_memory_usage: Cell<i32>,
	base_location: Cell<i32>,
	program_name: Cell<String>,
}

impl SymbolTable {
	pub fn new() -> SymbolTable {
		SymbolTable {
			symbols: vec![],
			memory_locations: vec![],
			starting_memory_location: Cell::new(-1),
			first_instruction: Cell::new(-1),
			total_memory_usage: Cell::new(-1),
			base_location: Cell::new(-1),
			program_name: Cell::new("".to_string()),
		}
	}
}

pub trait SymbolTablePublic {
	fn parse_symbol_table(&mut self, filename: &str);
}

impl SymbolTablePublic for SymbolTable {
	fn parse_symbol_table(&mut self, filename: &str) {
		if let Ok(lines) = util::read_lines(filename) {
			let mut line_number: i32 = 0;
			let mut current_memory_location: i32 = 0;

			for line in lines {
				if let Ok(line_str) = line {
					self.memory_locations.push(current_memory_location);

					line_number += 1;
					self.parse_line(line_str, line_number, &mut current_memory_location);
				}
			}
		}
	}
}

trait SymbolTablePrivate {
	fn parse_line(&mut self, line: String, line_number: i32, current_memory_location: &mut i32);
	fn handle_instruction(&self, line_number: i32, current_memory_location: &mut i32, instruction: &str, operand: Option<&str>);
	fn handle_directive(&self, line_number: i32, current_memory_location: &mut i32, directive: &str, operand: Option<&str>);
	fn add_symbol(&mut self, line_number: i32, name: &str, memory_location: i32);
}

impl SymbolTablePrivate for SymbolTable {
	fn parse_line(&mut self, line: String, line_number: i32, current_memory_location: &mut i32) {
		// ignore comments
		if line.starts_with("#") {
			return;
		}

		println!("{}", line);
		let split: Vec<String> = sic_line_to_vector(line);

		if split.len() == 0 {
			println!("Error (line {}): Empty line! Not allowed in SIC. Use comments instead (#)", line_number);
			exit(1);
		} else if split.len() == 1 {
			let str1 = split.get(0).unwrap();
			if !is_instruction(str1) {
				println!("Error (line {}): Not an instruction!", line_number);
				exit(1);
			}

			self.handle_instruction(line_number, current_memory_location, str1, None);
		} else if split.len() == 2 {
			let str1 = split.get(0).unwrap();
			let str2 = split.get(1).unwrap();

			if is_instruction(str1) {
				self.handle_instruction(line_number, current_memory_location, str1, Some(str2));
			} else if is_instruction(str2) {
				self.handle_instruction(line_number, current_memory_location, str2, None);
				self.add_symbol(line_number, str1, *current_memory_location);
			} else if is_directive(str1) {
				self.handle_directive(line_number, current_memory_location, str1, Some(str2))
			} else if is_directive(str2) {
				self.handle_directive(line_number, current_memory_location, str2, None);
				self.add_symbol(line_number, str1, *current_memory_location);
			} else {
				println!("Error (line {}): Invalid line! Not an instruction or directive!", line_number);
				exit(1);
			}
		} else {
			let str1 = split.get(0).unwrap();
			let str2 = split.get(1).unwrap();
			let str3 = split.get(2).unwrap();

			if is_instruction(str2) {
				self.handle_instruction(line_number, current_memory_location, str2, Some(str3));
				self.add_symbol(line_number, str1, *current_memory_location);
			} else if is_directive(str2) {
				self.handle_directive(line_number, current_memory_location, str2, Some(str3));
				self.add_symbol(line_number, str1, *current_memory_location);
			} else {
				println!("Error (line {}): Invalid line! Not an instruction or directive!", line_number);
				exit(1);
			}
		}
	}

	fn handle_instruction(&self, line_number: i32, current_memory_location: &mut i32, instruction: &str, operand: Option<&str>) {}

	fn handle_directive(&self, line_number: i32, current_memory_location: &mut i32, directive: &str, operand: Option<&str>) {
		match directive {
			"START" => {
				let location = parse_str_i32_or_error(operand, 16, format!("error (line {}): invalid or no operand provided for directive.", line_number));
				self.starting_memory_location.set(location);
			}
			"BYTE" => {
				if operand.is_none() {
					println!("Error (line {}): Invalid or no operand provided for directive.", line_number);
					exit(1);
				}
				let operand_string = String::from(operand.unwrap());
				if operand_string.starts_with("C'") && operand_string.ends_with("'") {
					let stripped = operand_string.strip_prefix("C'").unwrap();
					let num_bytes: i32 = stripped.len() as i32;
					*current_memory_location += num_bytes;
				} else if operand_string.starts_with("X'") && operand_string.ends_with("'") {
					let stripped = operand_string.strip_prefix("X'").unwrap();
					let num_bytes: i32 = (stripped.len() / 2 + stripped.len() % 2) as i32;
					*current_memory_location += num_bytes;
				} else {
					println!("Error (line {}): Invalid or no operand provided for directive.", line_number);
					exit(1);
				}
			}
			"WORD" => {
				let word = parse_str_i32_or_error(operand, 10, format!("Error (line {}): Invalid or no operand provided for directive.", line_number));
				if word > 8388607 || word < -8388608 {
					println!("Error (line {}): Invalid word value provided! Outside of 24 bit limit.", line_number);
					exit(1);
				}
				*current_memory_location += 3;
			}
			"RESB" => {
				let num_bytes = parse_str_i32_or_error(operand, 10, format!("Error (line {}): Invalid or no operand provided for directive.", line_number));
				*current_memory_location += num_bytes;
			}
			"RESW" => {
				let num_words = parse_str_i32_or_error(operand, 10, format!("Error (line {}): Invalid or no operand provided for directive.", line_number));
				*current_memory_location += num_words * 3;
			}
			"RESR" => {
				*current_memory_location += 3;
			}
			"EXPORTS" => {
				*current_memory_location += 3;
			}
			&_ => {}
		}
	}

	fn add_symbol(&mut self, line_number: i32, name: &str, memory_location: i32) {
		let str = String::from(name);

		// TODO: Add bad symbol checks
		let symbol = Symbol {
			name: str,
			memory_location,
		};
		self.symbols.push(symbol);
	}
}

fn sic_line_to_vector(line: String) -> Vec<String> {
	let mut temp: String = String::from("");
	let mut vector: Vec<String> = vec![];

	let mut in_string: bool = false;

	for c in line.chars() {
		if c == '\r' || c == '\n' {
			if temp.len() > 0 {
				vector.push(temp);
				temp = String::from("");
			}
		} else if c == ' ' || c == '\t' {
			if !in_string && temp.len() > 0 {
				vector.push(temp);
				temp = String::from("");
			} else if in_string {
				temp.push(c);
			}
		} else {
			if c == '\'' {
				in_string = !in_string;
			}
			temp.push(c);
		}
	}
	if temp.len() > 0 {
		vector.push(temp);
		temp = String::from("");
	}

	vector
}

fn parse_str_i32_or_error(str: Option<&str>, base: u32, error_message: String) -> i32 {
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