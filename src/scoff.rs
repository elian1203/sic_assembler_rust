use std::process::exit;
use hex;

use crate::instructions::*;
use crate::symbols::*;
use crate::util::*;

pub fn write_object_file(filename: &str, symbol_table: SymbolTable) {
	if let Ok(lines) = read_lines(filename) {
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
				get_directive_code(&symbol_table, line_number, str1.unwrap(), str2);
			} else if str2.is_some() && is_instruction(str2.unwrap()) {

			} else if str2.is_some() && is_directive(str2.unwrap()) {
				get_directive_code(&symbol_table, line_number, str2.unwrap(), str3);
			} else {
				println!("Error (line {}): Could not parse line!", line_number);
				exit(1);
			}
		}
	}
}

fn get_instruction_code(symbol_table: &SymbolTable, line_number: usize,
                        opcode: &String, operand: Option<&String>,
                        modifications: &mut Vec<String>) -> String {
	let current_memory_location = symbol_table.memory_locations.get(line_number).unwrap();
	let opcode_hex = get_instruction_hex(&opcode);
	todo!()
}

fn get_directive_code(symbol_table: &SymbolTable, line_number: usize, directive: &String,
                      operand: Option<&String>) {
	let mut current_memory_location = symbol_table.memory_locations.get(line_number).unwrap().clone();

	let base_code: String = match directive.as_str() {
		"BYTE" => {
			let operand = operand.unwrap();
			let mut str = if operand.starts_with("C'") && operand.ends_with("'") {
				let stripped = operand.strip_prefix("C'").unwrap().strip_suffix("'").unwrap();
				hex::encode_upper(stripped)
			} else {
				let stripped = operand.strip_prefix("X'").unwrap().strip_suffix("'").unwrap();
				stripped.to_owned()
			};

			if str.len() % 2 != 0 {
				println!("add 0");
				let mut temp = "0".to_owned();
				temp.push_str(str.as_str());
				str = temp;
			}
			str
		}
		"WORD" => {
			let word = parse_string_i32_or_error(operand, 10,
				format!("Error (line {}): Invalid word operand provided!", line_number));
			format!("{:0>6X}", word)
		}
		"END" => {
			if operand.is_some() && symbol_table.get_symbol_location(&operand.unwrap()) == -1 {
				println!("Error (line {}): End directive has invalid symbol!", line_number);
				exit(1);
			}
			String::new()
		}
		"BASE" => {
			symbol_table.base_location.set(current_memory_location);
			String::new()
		}
		&_ => {
			String::new()
		}
	};

	let mut directive_code = String::new();
	let mut base_code = base_code.as_str();

	while base_code.len() > 0 {
		let len_appended = if base_code.len() > 60 { 60 } else { base_code.len() };
		let bytes_appended: i32 = (len_appended / 2) as i32;
		let append = format!("T{:0>6X}{:0>2X}{}\n", current_memory_location, bytes_appended, &base_code[..len_appended]);
		directive_code.push_str(&append);
		base_code = &base_code[len_appended..];
		current_memory_location += bytes_appended;
	}
}
