use regex::Regex;
use std::{io, io::prelude::*};
fn main() {
    let re = Regex::new(r"^\D*(\d).*(\d)\D*$").unwrap();
    let re2 = Regex::new(r"(\d)").unwrap();
    let mut result: u32 = 0;
    for line in io::stdin().lock().lines() {
        let current = line.unwrap();
        let nums = re.captures(&current);
        match nums {
            Some(capture) => {
                result +=
                    &capture[1].parse::<u32>().unwrap() * 10 + &capture[2].parse::<u32>().unwrap()
            }
            None => {
                if let Some(num) = re2.captures(&current) {
                    result += &num[1].parse::<u32>().unwrap() * 11;
                };
            }
        }
    }
    println!("{}", result);
}
