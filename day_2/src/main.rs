// Nicholas J Uhlhorn
// Advent of Code
// Day 2 - Bags
// Input -> List of Games and blocks shown
//          e.g.: Game <ID>: <# color>, <#, color>; <#, color> ... etc
// Output <- sum of game record ids that are possible games
//           if there were 12 red, 13 green, and 14 blue cubes


use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use simple_error::*;

// Change the alias of Result
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}


#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
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

    let mut id_sum = 0;
    for line in lines.lines() {
        let game = build_game(line).unwrap();
        let mut valid_game = true;
        for round in game.rounds {
            if !check_validity(round, 12, 13, 14) {
                valid_game = false
            }
        }
        if valid_game {
            id_sum = id_sum + game.id;
        }
    }
    print!("{}\n", id_sum);
}

fn check_validity(round : Round, red_count : u32, green_count : u32, blue_count : u32) -> bool {
    return round.red <= red_count && round.green <= green_count && round.blue <= blue_count
}

fn build_game(line : &str) -> Result<Game> {
    let game_preamble;
    let raw_rounds;
    match line.split_once(':') {
        None => panic!("Could not parse game correctly: {}", line),
        Some(values) => (game_preamble, raw_rounds) = values,
    }
    
    // FIXME: possible unhandled panic from unwraping option
    let id_raw =  game_preamble.split_once(' ').unwrap().1;
    let id = id_raw.parse()?;

    let mut rounds : Vec<Round> = Vec::new();
    for raw_round in raw_rounds.split(';') {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        for part in raw_round.split(',') {
            let (raw_amount, color) = part.trim().split_once(' ').unwrap();
            let amount : u32 = raw_amount.parse::<u32>()?;
            match color {
                "red" => round.red = amount,
                "green" => round.green = amount,
                "blue" => round.blue = amount,
                _ => bail!("Unkown color, {}, used!", color),
            }
        }
        rounds.push(round);
    }
    
    let game = Game {
        id: id,
        rounds: rounds,
    };
    return Ok(game);
}