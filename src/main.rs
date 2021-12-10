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

    if let Ok(lines) = util::read_lines(&args[1]) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
