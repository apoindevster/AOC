use regex::Regex;
use std::{io, io::prelude::*};

fn parse_u32_from_input(input: &str) -> u32 {
    match input.parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            let parsed: u32 = match input {
                "zero" => 0,
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("NAN"),
            };
            parsed
        }
    }
}

fn main() {
    let str_form = r"zero|one|two|three|four|five|six|seven|eight|nine";
    let re = Regex::new(&format!(r"(\d|{0}).*(\d|{0})", str_form)).unwrap();
    let re2 = Regex::new(&format!(r"(\d|{0})", str_form)).unwrap();
    let mut result: u32 = 0;
    for line in io::stdin().lock().lines() {
        let current = line.unwrap();
        let nums = re.captures(&current);
        match nums {
            Some(capture) => {
                result +=
                    parse_u32_from_input(&capture[1]) * 10 + parse_u32_from_input(&capture[2]);
            }
            None => {
                if let Some(num) = re2.captures(&current) {
                    result += parse_u32_from_input(&num[1]) * 11;
                };
            }
        }
    }
    println!("{}", result);
}
