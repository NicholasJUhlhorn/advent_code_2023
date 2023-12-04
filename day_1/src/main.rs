// Nicholas J Uhlhorn
// Advent of Code 2023
// Day one - Practicing touch typing, touching up my Rust, and starting to learn vim
// Input  -> lines of text that have letters mixed with numbers
// Output <- first and last digit of each line concatinated into a two digit number
//					 then the sum of each of those numbers

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    // get runtime variables
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect number of variables");
    }
    let file_path = &args[1];

    // create path to file
    let path = Path::new(file_path);
    let display = path.display();

    // open file
    let mut file = match File::open(&path) {
       Err(why) => panic!("Could not open {}: {}", display, why),
       Ok(file) => file,  
    };

    // read the file
    let mut line = String::new();
    match file.read_to_string(&mut line) {
       Err(why) => panic!("Could not read {}: {}", display, why),
       Ok(_) => print!("{} contains:\n{}", display, line), 
    }
}