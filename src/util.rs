use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::process::exit;

pub(crate) fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file_result = File::open(filename);
    if file_result.is_err() {
        println!("Could not open file!");
        exit(1);
    } else {
        let file = file_result.unwrap();
        let reader = io::BufReader::new(file);
        return Ok(reader.lines());
    }
}
