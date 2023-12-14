// Nicholas J Uhlhorn
// Advent of code day 4
// Input -> a list of cards with winning numbers and a list of numbers on the card.
// Output <- the sum of points of each card, cards with no matches are 0
//           cards with one match are worth 1, and cards with more matches
//           are worth 2 times more for each point (offset powers of 2).

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashSet;

struct Card {
    win_set: HashSet<u32>,
    numbers: Vec<u32>
}

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

    let cards: Vec<Card>;
    match parse_cards(lines) {
        Ok(card_vec) => cards = card_vec,
        Err(error) => panic!("problem parsing cards: {}",error)
    }

    let points: Vec<u32>;
    match calculate_card_points(cards) {
        Ok(point_list) => points = point_list,
        Err(error) => panic!("problem calculating card points: {}", error)
    }
    let sum: u32 = points.iter().sum();    
    print!("Points: {}", sum);
}

fn parse_cards(card_string: String) -> Result<Vec<Card>> {
    panic!("not implemented")
}

fn calculate_card_points(cards: Vec<Card>) -> Result<Vec<u32>> {
    panic!("not implemented")
}
