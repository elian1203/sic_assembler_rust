use std::process::exit;

use hex;

use crate::instructions::*;
use crate::symbols::*;
use crate::util::*;

pub fn write_object_file(filename: &str, mut symbol_table: SymbolTable) {
	if let Ok(lines) = read_lines(filename) {
		let mut text_records: Vec<String> = vec![];
		let mut mod_records: Vec<String> = vec![];

		for (line_number, line) in lines.enumerate() {
			let line_number = line_number + 1;
			if line.is_err() {
				println!("Error (line {}): Could not read line!", line_number);
				exit(1);
			}

			let line = line.unwrap();

			if line.starts_with("#") {
				continue;
			}

			let words = sic_line_to_vector(line);

			let str1 = words.get(0);
			let str2 = words.get(1);
			let str3 = words.get(2);

			let text_record = if str1.is_some() && is_instruction(str1.unwrap()) {
				get_instruction_code(&symbol_table, line_number, str1.unwrap(), str2, &mut mod_records)
			} else if str1.is_some() && is_directive(str1.unwrap()) {
				get_directive_code(&mut symbol_table, line_number, str1.unwrap(), str2)
			} else if str2.is_some() && is_instruction(str2.unwrap()) {
				get_instruction_code(&symbol_table, line_number, str2.unwrap(), str3, &mut mod_records)
			} else if str2.is_some() && is_directive(str2.unwrap()) {
				get_directive_code(&mut symbol_table, line_number, str2.unwrap(), str3)
			} else {
				println!("Error (line {}): Could not parse line!", line_number);
				exit(1);
			};

			if text_record.len() > 0 {
				text_records.push(text_record);
			}
		}

		let mut object_records: Vec<String> = vec![];
		let program_name = symbol_table.program_name;
		object_records.push(format!("H{: <7}{:0>6X}{:0>6X}", program_name,
		                            symbol_table.starting_memory_location,
		                            symbol_table.total_memory_usage));
		object_records.extend(text_records);
		object_records.extend(mod_records);
		object_records.push(format!("E{:0<6X}", symbol_table.first_instruction));

		let mut output_file = String::from(filename);
		output_file.push_str(".obj");

		write_lines(output_file, object_records);
	}
}

fn get_instruction_code(symbol_table: &SymbolTable, line_number: usize,
                        opcode: &String, operand: Option<&String>,
                        modifications: &mut Vec<String>) -> String {
	let current_memory_location = symbol_table.memory_locations.get(line_number - 1).unwrap();
	let opcode_hex = get_instruction_hex(&opcode);

	let instruction_format = get_instruction_format(&*opcode);

	let mut hash_symbol = false;
	let mut at_symbol = false;
	let mut x_index = false;

	let mut operand = if operand.is_some() { operand.unwrap().clone() } else { "".to_owned() };

	if operand.starts_with("#") {
		hash_symbol = true;
		operand = operand.as_str()[1..].to_owned();
	} else if operand.starts_with("#") {
		at_symbol = true;
		operand = operand.as_str()[1..].to_owned();
	}

	if operand.contains(",") {
		operand = operand.split(",").collect::<Vec<&str>>()[0].to_string();
		x_index = true;
	}

	match instruction_format {
		1 => {
			// format 1
			format!("T{:0>6X}01{:0>2X}", current_memory_location, opcode_hex)
		}
		2 => {
			// format 2
			let mut r1 = 0;
			let mut r2 = 0;

			for (index, char) in operand.chars().enumerate() {
				if (index == 1 && char != ',') || index == 3 {
					println!("Error (line {}): Invalid registers specified! Missing ',' or too many characters.", line_number);
					exit(1);
				}
				if index == 1 {
					continue;
				}

				let register = match char {
					'A' => { 0 }
					'X' => { 1 }
					'L' => { 2 }
					'B' => { 3 }
					'S' => { 4 }
					'T' => { 5 }
					'F' => { 6 }
					_ => {
						println!("Error (line {}): Invalid registers specified!", line_number);
						exit(1);
					}
				};

				if index == 0 {
					r1 = register;
				} else {
					r2 = register;
				}
			}
			format!("T{:0>6X}02{:0>2X}{}{}", current_memory_location, opcode_hex, r1, r2)
		}
		3 => {
			// format 3
			let mut first_byte = opcode_hex;

			if hash_symbol {
				// i bit flipped
				first_byte += 1;
			} else if at_symbol {
				// n bit flipped
				first_byte += 2;
			} else {
				// n & i bits flipped
				first_byte += 3;
			}

			let symbol_location = symbol_table.get_symbol_location(&operand);

			let mut displacement = if hash_symbol && symbol_location == -1 {
				let memory_address_input = parse_str_i32_or_error(Some(&operand), 10,
				                                                  format!("Error (line {}): Invalid symbol provided!", line_number));
				memory_address_input
			} else if symbol_location == -1 {
				// RSUB with no operand
				0
			} else {
				let program_counter = current_memory_location + 3;
				let program_counter_displacement = symbol_location - program_counter;
				let base_displacement = if symbol_table.base_location == -1 { 4096 } else { symbol_location - symbol_table.base_location };

				if program_counter_displacement >= -2048 && program_counter_displacement < 2048 {
					// use pc-relative addressing
					let mut p_bit = 8192;
					if program_counter_displacement < 0 {
						p_bit += 4096;
					}
					program_counter_displacement + p_bit
				} else if base_displacement >= 0 && base_displacement < 4096 {
					// use base-relative addressing
					let b_bit = 16384;
					base_displacement + b_bit
				} else {
					symbol_location
				}
			};

			if x_index {
				// use x-indexing
				displacement += 32768;
			}

			format!("T{:0>6X}03{:0>2X}{:0>4X}", current_memory_location, first_byte, displacement)
		}
		_ => {
			// format 4
			let mut first_byte = opcode_hex;

			if hash_symbol {
				// i bit flipped
				first_byte += 1;
			} else if at_symbol {
				// n bit flipped
				first_byte += 2;
			} else {
				// n & i bits flipped
				first_byte += 3;
			}

			let symbol_location = symbol_table.get_symbol_location(&operand);

			let mut displacement = if hash_symbol && symbol_location == -1 {
				let memory_address_input = parse_str_i32_or_error(Some(&operand), 10,
				                                                  format!("Error (line {}): Invalid symbol provided!", line_number));
				memory_address_input
			} else {
				symbol_location
			};

			if x_index {
				// use x-indexing
				displacement += 8388608;
			}

			// flip e bit
			displacement += 1048576;

			format!("T{:0>6X}04{:0>2X}{:0>6X}", current_memory_location, first_byte, displacement)
		}
	}
}

fn get_directive_code(symbol_table: &mut SymbolTable, line_number: usize, directive: &String,
                      operand: Option<&String>) -> String {
	let mut current_memory_location = symbol_table.memory_locations.get(line_number - 1).unwrap().clone();

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
			if !operand.is_some() {
				println!("Error (line {}): Base directive has no symbol!", line_number);
				exit(1);
			}

			let symbol_location = symbol_table.get_symbol_location(operand.unwrap());
			if symbol_location == -1 {
				println!("Error (line {}): Base directive has invalid symbol!", line_number);
				exit(1);
			}

			symbol_table.base_location = symbol_location;
			String::new()
		}
		&_ => {
			String::new()
		}
	};

	let mut directive_code = String::new();
	let mut base_code = base_code.as_str();

	while base_code.len() > 0 {
		if !directive_code.is_empty() {
			directive_code.push('\n');
		}

		let len_appended = if base_code.len() > 60 { 60 } else { base_code.len() };
		let bytes_appended: i32 = (len_appended / 2) as i32;
		let append = format!("T{:0>6X}{:0>2X}{}", current_memory_location, bytes_appended, &base_code[..len_appended]);
		directive_code.push_str(&append);
		base_code = &base_code[len_appended..];
		current_memory_location += bytes_appended;
	}

	directive_code
}
