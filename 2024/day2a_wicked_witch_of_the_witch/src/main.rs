use std::io::{self, prelude::*};

fn check_for_valid(iter: &mut impl Iterator<Item = i32>) -> bool {
    let mut n1 = iter.nth(0).unwrap();

    for n2 in iter.by_ref() {
        if n1 >= n2 || n2 - n1 > 3 {
            return false;
        }
        n1 = n2;
    }

    return true;
}

fn main() {
    let mut valid = 0;
    for line in io::stdin().lock().lines() {
        let mut parsed = line
            .as_ref()
            .unwrap()
            .split_ascii_whitespace()
            .map(|i| i.parse::<i32>().unwrap());
        let mut result = false;

        let mut peek = parsed.clone().peekable(); 

        if peek.nth(1) >= peek.nth(0) {
            result = check_for_valid(&mut parsed.rev());
        } else {
            result = check_for_valid(&mut parsed);
        }

        if result {
            valid += 1;
        }

    }
    print!("{}", valid);
}
