extern crate aoc2020;

use aoc2020::common::read_lines;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Policy {
    char: char,
    lo: i32,
    hi: i32,
    password: String,
}

impl FromStr for Policy {
    type Err = std::string::ParseError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let dash = line.find('-').unwrap();
        let whitespace = line.find(' ').unwrap();
        let semicolon = line.find(':').unwrap();

        let lo = line[0..dash].parse::<i32>().unwrap();
        let hi = line[dash + 1..whitespace].parse::<i32>().unwrap();
        let char = line.chars().nth(whitespace + 1).unwrap();
        let password = String::from_str(&line[semicolon + 2..]).unwrap();

        Ok(Policy { char, lo, hi, password })
    }
}

impl Policy {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.char).count() as i32;
        self.lo <= count && count <= self.hi
    }
}

fn main() {
    if let Ok(lines) = read_lines("../input/02.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                let policy = Policy::from_str(&ip).unwrap();
                if policy.is_valid() {
                    count += 1;
                }
            }
        }
        println!("{}", count);
    }
}
