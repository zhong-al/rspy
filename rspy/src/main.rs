use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    
    if let Err(e) = print_file_lines(filename) {
        eprintln!("Error reading {}: {}", filename, e);
    }
}

fn print_file_lines<P: AsRef<Path>>(filename: P) -> io::Result<()> {
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
