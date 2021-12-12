mod util;
mod instructions;
mod symbols;
mod scoff;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a SIC source file to assemble!");
        exit(0);
    }

    let filename = &args[1];

    let mut symbol_table = symbols::SymbolTable::new();
    symbol_table.parse_symbol_table(filename);
    // symbol_table.print_symbol_table();

    scoff::write_object_file(filename, symbol_table);
}
