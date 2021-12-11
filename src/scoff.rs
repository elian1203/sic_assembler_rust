use std::process::exit;

use crate::instructions::*;
use crate::symbols::*;
use crate::util;

pub fn write_object_file(filename: &str, symbol_table: SymbolTable) {
	if let Ok(lines) = util::read_lines(filename) {
		let text_records: Vec<String> = vec![];
		let mod_records: Vec<String> = vec![];

		for (line_number, line) in lines.enumerate() {
			if line.is_err() {
				println!("Error (line {}): Could not read line!", line_number);
				exit(1);
			}

			let line = line.unwrap();

			if line.starts_with("#") {
				continue;
			}

			let words = sic_line_to_vector(line);
			let num_words = words.len();

			let str1 = words.get(0);
			let str2 = words.get(1);
			let str3 = words.get(2);

			if str1.is_some() && is_instruction(str1.unwrap()) {

			} else if str1.is_some() && is_directive(str1.unwrap()) {

			} else if str2.is_some() && is_instruction(str2.unwrap()) {

			} else if str2.is_some() && is_directive(str2.unwrap()) {

			} else {
				println!("Error (line {}): Could not parse line!", line_number);
				exit(1);
			}
		}
	}
}

fn get_instruction_code(symbol_table: &SymbolTable, line_number: usize,
                        opcode: &String, operand: &String,
                        modifications: &mut Vec<String>) -> String {
	let current_memory_location = symbol_table.memory_locations.get(line_number).unwrap();
	let opcode_hex = get_instruction_hex(opcode);
	todo!()
}

