use regex::Regex;
use std::{io, io::prelude::*};

fn check_game(line: &str) -> bool {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut itr = re.find_iter(line);
    while let Some(single) = itr.next() {
        let (num, color) = single.as_str().split_once(" ").unwrap();
        let amount = num.parse::<u32>().expect("Failed to parse number of color");
        match color {
            "red" => {
                if amount > 12 {
                    return false;
                }
            }
            "green" => {
                if amount > 13 {
                    return false;
                }
            }
            "blue" => {
                if amount > 14 {
                    return false;
                }
            }
            _ => panic!("Failed to parse the color properly"),
        }
    }
    true
}

fn main() {
    let re = Regex::new(r"Game (\d+):").unwrap();
    let mut result = 0;
    for line in io::stdin().lock().lines() {
        let current = line.unwrap();
        let game_num = re.captures(&current);
        let Some(capture) = game_num else {
            panic!("Failed to parse the game num");
        };

        let num = match capture[1].parse::<u32>() {
            Ok(n) => n,
            _ => 0,
        };

        if check_game(&current) {
            result += num;
        }
    }
    println!("{}", result);
}
