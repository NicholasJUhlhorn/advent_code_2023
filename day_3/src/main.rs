// Nicholas J Uhlhorn
// Advent of Code
// Day 3 - Gear Ratios
// Input -> Grid of numbers, symbols, and periods
// Output <- The sum of all numbers that are adjacent to at least one symbol (even diagonals!) 

// Plan: 
// create flag matrix of symbols influence.
// run through matrix again dealing with the numbers,
// being very careful to not double count and keep number groups together! 
use array2d::Array2D;
use std::error;
use simple_error::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

    let flags;
    match make_flag_matrix(&lines) {
        Err(e) => panic!("encountered error with flag map: {}", e),
        Ok(map) => flags = map,
    }

    // for row in flags.as_rows(){
    //     for flag in row.iter(){
    //         print!("{}", *flag as u8);
    //     }
    //     print!("\n");
    // } 

    let ids = get_ids(&lines, &flags).unwrap();
    // print!("{:?}\n", ids);
    let id_sum : u32 = ids.iter().sum();
    print!("{}\n", id_sum);
}

fn get_ids(raw_string : &String, flags : &Array2D<bool>) -> Result<Vec<u32>> {
    let height = raw_string.lines().count();
    let width = raw_string.find('\n').unwrap();
    if height != flags.column_len() || width != flags.row_len() {
        bail!("string dimensions must match flag dimensions str:{},{}, flag:{},{}", width, height, flags.row_len(), flags.column_len());
    }
    let mut ids : Vec<u32> = Vec::new();
    let mut on_number = false;
    let mut current_number_str = String::new();
    let mut flagged = false;
    for (row, line) in raw_string.lines().enumerate() {
        // print!("{:?}\n", line.chars().collect::<Vec<_>>());
        // print!("{}\n", line);
        for flag in flags.row_iter(row).unwrap() {
            print!("{}", *flag as u8);
        }
        print!("\n");
        for (col, character) in line.chars().enumerate() {
            // Find next number
            if !on_number && !character.is_numeric() {
                // Skipping non numbers
                print!(".");
                continue;
            } 
            if on_number && !character.is_numeric() {
                // print!("\n");
                // add parse and add number to return list if so
                let mut p_val = "I";
                if flagged {
                    let current_number : u32 = current_number_str.parse().unwrap();
                    ids.push(current_number);
                    p_val = "P";
                }
                // End number stuff
                current_number_str = String::new();
                flagged = false;
                on_number = false;
                print!("{}", p_val);
            }
            if character.is_numeric() {
                // Start number stuff
                if !on_number {
                    on_number = true;
                }
                current_number_str.push(character);

                // check if digit is flagged
                let cur_flag : &bool;
                match flags.get(row, col) {
                    Some(b) => cur_flag = b,
                    None => panic!("no flag at {},{}", row, col), 
                }
                flagged = flagged || *cur_flag; 
                if flagged {
                    print!("F")
                } else {
                    print!("{}", character);
                }
                // print!("{}:{}\n", current_number_str, flagged);
            }
        }
        print!("\n");
    }

    return Ok(ids);
}

fn make_flag_matrix(raw_string : &String) -> Result<Array2D<bool>>{
    // get dimensions of raw string
    let height = raw_string.lines().count();
    let width;
    match raw_string.find('\n') {
        Some(index) => width = index,
        None => bail!("could not find new line in text\n"),
    }
    // print!("{},{}\n", width, height);
    let mut flags = Array2D::filled_with(false, height, width);
    // populate flags
    for (row_index, line) in raw_string.lines().enumerate() {
        for (col_index, character) in line.chars().enumerate() {
            // check for special symbol (not '.'s)
            if !character.is_alphanumeric() && character != '.' {
                // find range immediately around special character
                let (x_start, x_end) = (col_index-1, col_index+2);
                let (y_start, y_end) = (row_index-1, row_index+2);
                // fill flag map at those points               
                for x in x_start..x_end {
                    for y in y_start..y_end {
                        match flags.set(y, x, true){
                            Err(_) => (), // out of bounds, but that is okay
                            Ok(()) => (),
                        };
                    }
                }
            }
        }
    }
    return Ok(flags);
}