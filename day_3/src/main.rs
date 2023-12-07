// Nicholas J Uhlhorn
// Advent of Code
// Day 3 - Gear Ratios
// Input -> Grid of numbers, symbols, and periods
// Output <- The sum of all numbers that are adjacent to at least one symbol (even diagnals!) 

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
use std::cmp;


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
        Err(e) => panic!("encoured error with flag map: {}", e),
        Ok(map) => flags = map,
    }

    // for row in flags.as_rows(){
    //     for flag in row.iter(){
    //         print!("{}", *flag as u8);
    //     }
    //     print!("\n");
    // } 

    let ids = get_ids(&lines, &flags).unwrap();
    let id_sum : u32 = ids.iter().sum();
    print!("{}\n", id_sum);
}

fn get_ids(raw_string : &String, flags : &Array2D<bool>) -> Result<Vec<u32>> {
    let height = raw_string.lines().count();
    let width = raw_string.find('\n').unwrap() - 1;
    if height != flags.column_len() || width != flags.row_len() {
        bail!("string dimentions must match flag dimentions str:{},{}, flag:{},{}", width, height, flags.row_len(), flags.column_len());
    }
    let mut ids : Vec<u32> = Vec::new();
    let mut on_number = false;
    let mut current_number_str = String::new();
    let mut flaged = false;
    for (row, line) in raw_string.lines().enumerate() {
        // print!("{}\n", line);
        // for flag in flags.row_iter(row).unwrap() {
        //     print!("{}", *flag as u8);
        // }
        // print!("\n");
        for (col, character) in line.chars().enumerate() {
            // Find next number
            if !on_number && !character.is_numeric() {
                // Skiping non numbers
                continue;
            } 
            if on_number && !character.is_numeric() {
                // print!("\n");
                // add parse and add number to return list if so
                if flaged {
                    let current_number : u32 = current_number_str.parse().unwrap();
                    ids.push(current_number);
                }
                // End number stuff
                current_number_str = String::new();
                flaged = false;
                on_number = false;
            }
            if character.is_numeric() {
                // Start number stuff
                if !on_number {
                    on_number = true;
                }
                current_number_str.push(character);

                // check area around number for flags
                flaged = flaged || *flags.get(row, col).unwrap(); 
                // print!("{}:{}\n", current_number_str, flaged);
            }
        }
    }

    return Ok(ids);
}

fn make_flag_matrix(raw_string : &String) -> Result<Array2D<bool>>{
    // get dimentions of raw string
    let height = raw_string.lines().count();
    let width = raw_string.find('\n').unwrap() - 1;
    let mut flags = Array2D::filled_with(false, height, width);
    // populate flags
    for (row_index, line) in raw_string.lines().enumerate() {
        for (col_index, character) in line.chars().enumerate() {
            // check for special symbol (not '.'s)
            if !character.is_alphanumeric() && character != '.' {
                // find range imediatly around special character
                // FIXME: possible wrong logic for incrementing end by 2 not 1
                let (x_start, x_end) = (cmp::max(0, col_index as i32-1) as usize, cmp::min(col_index as i32+2, width as i32) as usize);
                let (y_start, y_end) = (cmp::max(0, row_index as i32-1) as usize, cmp::min(row_index as i32+2, height as i32) as usize);
                // fill flagmap at those points               
                for x in x_start..x_end {
                    for y in y_start..y_end {
                        match flags.set(y, x, true){
                            Err(e) => bail!("error filling flag_matrix: {:?}", e),
                            Ok(()) => (),
                        };
                    }
                }
            }
        }
    }
    return Ok(flags);
}