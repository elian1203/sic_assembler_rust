use std::cell::Cell;
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
		println!("{}", line);
		let split: Vec<&str> = line.split_whitespace().collect();

		if split.len() == 0 {
			println!("Error (line {}): Empty line! Not allowed in SIC. Use comments instead (#)", line_number);
			exit(1);
		} else if split.len() == 1 {
			let str1 = split[0];
			if !is_instruction(str1) {
				println!("Error (line {}): Not an instruction!", line_number);
				exit(1);
			}

			self.handle_instruction(line_number, current_memory_location, str1, None);
		} else if split.len() == 2 {
			let str1 = split[0];
			let str2 = split[1];

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
			let str1 = split[0];
			let str2 = split[1];
			let str3 = split[2];

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

	fn handle_directive(&self, line_number: i32, current_memory_location: &mut i32, directive: &str, operand: Option<&str>) {}

	fn add_symbol(&mut self, line_number: i32, name: &str, memory_location: i32) {
		let str = String::from(name);
		let symbol = Symbol {
			name: str,
			memory_location,
		};
		self.symbols.push(symbol);
	}
}
