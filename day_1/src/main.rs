// Nicholas J Uhlhorn
// Advent of Code 2023
// Day one - Practicing touch typing, touching up my Rust, and starting to learn vim
// Input  -> lines of text that have letters mixed with numbers
// Output <- first and last digit of each line concatenated into a two digit number
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
    let mut lines = String::new();
    match file.read_to_string(&mut lines) {
       Err(why) => panic!("Could not read {}: {}", display, why),
       Ok(_) => (), 
    }

    let mut sum = 0;

    for line in lines.lines() {
        sum = sum + parse_line(line);
    }

    print!("sum: {}\n", sum);
}

fn parse_line(line : &str) -> u32 {
    let mut first_digit_char = ' ';
    let mut last_digit_char = ' ';
    let mut string_slice = line;
    while string_slice.len() > 0 {
        match check_numeric(string_slice) {
           Some(digit_char) => {
            if  first_digit_char == ' ' {
                first_digit_char = digit_char;
            }
            last_digit_char = digit_char;
           },
           None => (),
        }
        (_, string_slice) = string_slice.split_at(1);
    }
    let combined_digits_string = format!("{}{}", first_digit_char, last_digit_char);
    
    match combined_digits_string.parse() {
        Err(why) => panic!("Could not parse digits \"{}\": {}", combined_digits_string, why),
        Ok(value) => return value,
    }
}

fn check_numeric(string_slice : &str) -> Option<char>{
    // check first character for number
    let first_char = string_slice.chars().next().unwrap();
    if first_char.is_numeric() {
        return Some(first_char);
    }
    // check for string match
    check_printed_digit(string_slice)
}

fn check_printed_digit(string_slice : &str) -> Option<char> {
    if string_slice.starts_with("one") {
        return Some('1');
    }
    if string_slice.starts_with("two") {
        return Some('2');
    }
    if string_slice.starts_with("three") {
        return Some('3');
    }
    if string_slice.starts_with("four") {
        return Some('4');
    }   
    if string_slice.starts_with("five") {
        return Some('5');
    }
    if string_slice.starts_with("six") {
        return Some('6');
    }
    if string_slice.starts_with("seven") {
        return Some('7');
    }
    if string_slice.starts_with("eight") {
        return Some('8');
    }
    if string_slice.starts_with("nine") {
        return Some('9');
    }
    return None
}
