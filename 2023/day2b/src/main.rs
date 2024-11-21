use regex::Regex;
use std::{io, io::prelude::*};

struct Counts {
    red: u32,
    green: u32,
    blue: u32,
}

fn get_game_power(line: &str) -> u32 {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let mut mins = Counts {
        red: 0,
        green: 0,
        blue: 0,
    };

    let mut itr = re.find_iter(line);
    while let Some(single) = itr.next() {
        let (num, color) = single.as_str().split_once(" ").unwrap();
        let amount = num.parse::<u32>().expect("Failed to parse number of color");
        match color {
            "red" => {
                if amount > mins.red {
                    mins.red = amount;
                }
            }
            "green" => {
                if amount > mins.green {
                    mins.green = amount;
                }
            }
            "blue" => {
                if amount > mins.blue {
                    mins.blue = amount;
                }
            }
            _ => panic!("Failed to parse the color properly"),
        }
    }
    mins.red * mins.green * mins.blue
}

fn main() {
    let mut result = 0;
    for line in io::stdin().lock().lines() {
        let current = line.unwrap();
        result += get_game_power(&current);
    }
    println!("{}", result);
}
