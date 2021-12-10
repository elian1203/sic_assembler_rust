mod util;
mod instructions;
mod symbols;
mod scoff;

use std::env;
use std::process::exit;
use crate::symbols::SymbolTablePublic;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a SIC source file to assemble!");
        exit(0);
    }

    let mut symbol_table = symbols::SymbolTable::new();
    symbol_table.parse_symbol_table(&args[1]);
}
