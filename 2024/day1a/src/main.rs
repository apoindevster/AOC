use regex::Regex;
use sorted_vec::SortedVec;
use std::{io, io::prelude::*};

fn main() {
    let re = Regex::new(r"(\d+)\D*(\d+)$").unwrap();
    let mut v1: SortedVec<i32> = SortedVec::new();
    let mut v2: SortedVec<i32> = SortedVec::new();
    for line in io::stdin().lock().lines() {
        let curr = line.unwrap();
        let nums = re.captures(&curr);
        match nums {
            Some(capture) => {
                v1.insert(capture[1].parse::<i32>().unwrap());
                v2.insert(capture[2].parse::<i32>().unwrap());
            }
            None => panic!("Failed to parse the numbers from the input"),
        }
    }

    let mut total: i32 = 0;
    for (pos, elem) in v1.iter().enumerate() {
        let diff = elem - v2[pos];
        total += diff.abs();
    }

    println!("{}", total)
}
