// Nicholas J Uhlhorn
// Advent of code day 4
// Input -> a list of cards with winning numbers and a list of numbers on the card.
// Output <- the sum of points of each card, cards with no matches are 0
//           cards with one match are worth 1, and cards with more matches
//           are worth 2 times more for each point (offset powers of 2).

use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::collections::HashSet;

#[derive(Debug)]
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
    print!("Points: {}\n", sum);
}

fn parse_cards(card_string: String) -> Result<Vec<Card>> {
    let mut card_list: Vec<Card> = Vec::new();
    for card_line in card_string.lines() {
        // parse the numbers out of the line
        let first_split = card_line.split_once(":").context("could not parse a card line at ':'")?;
        let number_string = first_split.1;
        let second_split = number_string.split_once('|').context("could not parse a card line at '|'.")?;
        let win_number_string =  second_split.0.trim().split_ascii_whitespace();
        let match_numbers = second_split.1.trim().split_ascii_whitespace();

        // create card to add 
        let mut win_set: HashSet<u32> = HashSet::new();
        for number_string in win_number_string.into_iter() {
            let value: u32 = number_string.parse()?;
            win_set.insert(value);
        }

        let mut match_list: Vec<u32> = Vec::new();
        for number_string in match_numbers.into_iter() {
            let value: u32 = number_string.parse()?;
            match_list.push(value);
        }

        let new_card = Card {
            win_set: win_set,
            numbers: match_list
        };

        card_list.push(new_card);
    }

    return Ok(card_list);
}

fn calculate_card_points(cards: Vec<Card>) -> Result<Vec<u32>> {
    let mut points: Vec<u32> = Vec::new();
    for card in cards {
        let mut matches = 0;
        for number in card.numbers {
            if card.win_set.contains(&number) {
                matches += 1;
            }
        }
        if matches == 0 {
            points.push(0);
        }else {
            let base: u32 = 2;
            let card_points = base.pow(matches - 1);
            points.push(card_points);
        }
    }
    return Ok(points);
}
