use std::cell::Cell;
use std::process::exit;
use std::string::String;

use crate::instructions;
use crate::instructions::*;
use crate::util;

struct Symbol {
	name: String,
	memory_location: Cell<i32>,
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
	fn contains_symbol(&self, name: &str) -> bool;
	fn print_symbol_table(&self);
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

			if current_memory_location > 1048576 {
				println!("Error (line {}): SIC memory exceeded!", line_number);
				exit(1);
			}

			if self.starting_memory_location.get() == -1 {
				println!("Error (line {}): No START directive found!", line_number);
				exit(1);
			}

			self.total_memory_usage.set(current_memory_location);

			for symbol in &self.symbols {
				symbol.memory_location.set(symbol.memory_location.get() + self.starting_memory_location.get());
			}
		}
	}

	fn contains_symbol(&self, name: &str) -> bool {
		for symbol in &self.symbols {
			if symbol.name == name {
				return true;
			}
		}
		return false;
	}

	fn print_symbol_table(&self) {
		for symbol in &self.symbols {
			println!("{: >6}\t{:X}", symbol.name, symbol.memory_location.get());
		}
	}
}

trait SymbolTablePrivate {
	fn parse_line(&mut self, line: String, line_number: i32, current_memory_location: &mut i32);
	fn handle_instruction(&self, current_memory_location: &mut i32, instruction: &str);
	fn handle_directive(&self, line_number: i32, current_memory_location: &mut i32, directive: &str, operand: Option<&str>);
	fn add_symbol(&mut self, line_number: i32, name: &str, memory_location: i32);
}

impl SymbolTablePrivate for SymbolTable {
	fn parse_line(&mut self, line: String, line_number: i32, current_memory_location: &mut i32) {
		// ignore comments
		if line.starts_with("#") {
			return;
		}

		// println!("{}", line);
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

			self.handle_instruction(current_memory_location, str1);
		} else if split.len() == 2 {
			let str1 = split.get(0).unwrap();
			let str2 = split.get(1).unwrap();

			if is_instruction(str1) {
				self.handle_instruction(current_memory_location, str1);
			} else if is_instruction(str2) {
				self.add_symbol(line_number, str1, *current_memory_location);
				self.handle_instruction(current_memory_location, str2);
			} else if is_directive(str1) {
				self.handle_directive(line_number, current_memory_location, str1, Some(str2))
			} else if is_directive(str2) {
				self.add_symbol(line_number, str1, *current_memory_location);
				self.handle_directive(line_number, current_memory_location, str2, None);
			} else {
				println!("Error (line {}): Invalid line! Not an instruction or directive!", line_number);
				exit(1);
			}
		} else {
			let str1 = split.get(0).unwrap();
			let str2 = split.get(1).unwrap();
			let str3 = split.get(2).unwrap();

			if is_instruction(str2) {
				self.add_symbol(line_number, str1, *current_memory_location);
				self.handle_instruction(current_memory_location, str2);
			} else if is_directive(str2) {
				self.add_symbol(line_number, str1, *current_memory_location);
				self.handle_directive(line_number, current_memory_location, str2, Some(str3));

				if str2 == "START" {
					self.program_name.set(str2.clone());
				}
			} else {
				println!("Error (line {}): Invalid line! Not an instruction or directive!", line_number);
				exit(1);
			}
		}
	}

	fn handle_instruction(&self, current_memory_location: &mut i32, instruction: &str) {
		if self.first_instruction.get() == -1 {
			self.first_instruction.set(*current_memory_location);
		}
		*current_memory_location += get_instruction_format(instruction);
	}

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

		let first_char = str.chars().next().unwrap();

		if !first_char.is_alphabetic() || !first_char.is_uppercase() {
			println!("Error (line {}): Symbol must start with uppercase alpha character.", line_number);
			exit(1);
		} else if str.len() > 6 {
			println!("Error (line {}): Symbol greater than max length (6)", line_number);
			exit(1);
		} else if str.contains("$") || str.contains("!") || str.contains("=")
			|| str.contains("+") || str.contains("-") || str.contains("(")
			|| str.contains(")") || str.contains("@") {
			println!("Error (line {}): Symbol contains illegal characer", line_number);
			exit(1);
		} else if is_directive(str.as_str()) {
			println!("Error (line {}): Symbol cannot be a directive name!", line_number);
			exit(1);
		} else if self.contains_symbol(str.as_str()) {
			println!("Error (line {}): Symbol already exists!", line_number);
			exit(1);
		}

		for c in str.chars() {
			if c.is_alphabetic() && !c.is_uppercase() {
				println!("Error (line {}): Symbol cannot contain lowercase letters!", line_number);
				exit(1);
			}
		}
		let location = Cell::new(memory_location);

		let symbol = Symbol {
			name: str,
			memory_location: location,
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