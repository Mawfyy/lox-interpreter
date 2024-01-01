mod scanner;
mod types;
use clap::Parser;
use scanner::{scan_tokens, Errors};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::process::exit;
use types::Args;

fn main() {
    let args = Args::parse();

    let Ok(file) = File::open(&args.path) else {
        println!("Error {:#?}", Errors::FileNotFound);
        exit(1)
    };
    let mut reader = BufReader::new(file);
    let mut file_content = String::new();
    if let Err(error) = reader.read_to_string(&mut file_content) {
        println!("Error {:#?}", error);
        exit(1)
    }

    if let Err(Errors::SyntaxError(error_syntax)) = scan_tokens(file_content) {
        println!("{:#?}", error_syntax);
        exit(1)
    }
}
