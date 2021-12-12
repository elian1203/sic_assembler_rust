const INSTRUCTIONS: &'static [(&str, i32)] = &[
	("ADD", 0x18),
	("ADDF", 0x58),
	("ADDR", 0x90),
	("AND", 0x40),
	("CLEAR", 0xB4),
	("COMP", 0x28),
	("COMPF", 0x88),
	("COMPR", 0xA0),
	("DIV", 0x24),
	("DIVF", 0x64),
	("DIVR", 0x9C),
	("FIX", 0xC4),
	("FLOAT", 0xC0),
	("HIO", 0xF4),
	("J", 0x3C),
	("JEQ", 0x30),
	("JGT", 0x34),
	("JLT", 0x38),
	("JSUB", 0x48),
	("LDA", 0x00),
	("LDB", 0x68),
	("LDCH", 0x50),
	("LDF", 0x70),
	("LDL", 0x08),
	("LDS", 0x6C),
	("LDT", 0x74),
	("LDX", 0x04),
	("LPS", 0xD0),
	("MUL", 0x20),
	("MULF", 0x60),
	("MULR", 0x98),
	("NORM", 0xC8),
	("OR", 0x44),
	("RD", 0xD8),
	("RMO", 0xAC),
	("RSUB", 0x4C),
	("SHIFTL", 0xA4),
	("SHIFTR", 0xA8),
	("SIO", 0xF0),
	("SSK", 0xEC),
	("STA", 0x0C),
	("STB", 0x78),
	("STCH", 0x54),
	("STF", 0x80),
	("STI", 0xD4),
	("STL", 0x14),
	("STS", 0x7C),
	("STSW", 0xE8),
	("STT", 0x84),
	("STX", 0x10),
	("SUB", 0x1C),
	("SUBF", 0x5C),
	("SUBR", 0x94),
	("SVC", 0xB0),
	("TD", 0xE0),
	("TIO", 0xF8),
	("TIX", 0x2C),
	("TIXR", 0xB8),
	("WD", 0xDC)
];

const FORMAT_1: &'static [&str] = &["FIX", "FLOAT", "HIO", "NORM", "SIO", "TIO"];
const FORMAT_2: &'static [&str] = &["ADDR", "CLEAR", "COMPR", "DIVR", "MULR", "RMO", "SHIFTL",
	"SHIFTR", "SUBR", "SVC", "TIXR"];

const DIRECTIVES: &'static [&str] = &["START", "END", "BYTE", "WORD", "RESB", "RESW", "RESR", "EXPORTS", "BASE"];

pub fn is_instruction(str: &str) -> bool {
	let str = str.trim_start_matches("+");
	for (instruction, _hex) in INSTRUCTIONS {
		if *instruction == str {
			return true;
		}
	}
	return false;
}

pub fn is_directive(str: &str) -> bool {
	return DIRECTIVES.contains(&str);
}

pub fn get_instruction_format(opcode: &str) -> i32 {
	let opcode_trimmed = opcode.trim_start_matches("+");

	return if opcode.starts_with("+") {
		4
	} else if FORMAT_1.contains(&opcode_trimmed) {
		1
	} else if FORMAT_2.contains(&opcode_trimmed) {
		2
	} else {
		3
	};
}

pub fn get_instruction_hex(opcode: &String) -> i32 {
	let opcode = opcode.trim_start_matches("+");
	for (instruction, hex) in INSTRUCTIONS {
		if *instruction == opcode {
			return *hex;
		}
	}
	return -1;
}