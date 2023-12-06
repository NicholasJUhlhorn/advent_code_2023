// Nicholas J Uhlhorn
// Advent of Code
// Day 3 - Gear Ratios
// Input -> Grid of numbers, symbols, and periods
// Output <- The sum of all numbers that are adjacent to at least one symbol (even diagnals!) 

// Plan: 
// create flag matrix of symbols influence.
// run through matrix again dealing with the numbers,
// being very careful to not double count and keep number groups together! 
use array2d::{Array2D, Error};
use std::error;
use simple_error::*;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


fn main() {

}

fn make_flag_matrix(raw_string : &String) -> Result<Array2D<bool>>{
    
    bail!("not implamented")
}