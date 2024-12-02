use regex::Regex;
use std::{collections::HashMap, io, io::prelude::*, vec::Vec};

fn main() {
    let re = Regex::new(r"(\d+)\D*(\d+)$").unwrap();
    let mut l1 = Vec::new();
    let mut map = HashMap::new();
    for line in io::stdin().lock().lines() {
        let curr = line.unwrap();
        let nums = re.captures(&curr);
        match nums {
            Some(capture) => {
                l1.push(capture[1].parse::<i32>().unwrap());

                let second = capture[2].parse::<i32>().unwrap();
                if let Some(curr) = map.get_mut(&second) {
                    *curr += 1;
                } else {
                    map.insert(second, 1);
                }
            }
            None => panic!("Failed to parse the numbers from the input"),
        }
    }

    let mut total: i32 = 0;
    for num in l1 {
        if let Some(curr) = map.get(&num) {
            total += *curr * num;
        }
    }

    println!("{}", total)
}
