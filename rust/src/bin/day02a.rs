extern crate aoc2020;

use aoc2020::common::read_lines;
use aoc2020::policy::Policy;
use std::str::FromStr;

fn is_valid(policy: &Policy) -> bool {
    let count = policy.password.matches(policy.char).count();
    policy.lo <= count && count <= policy.hi
}

fn main() {
    if let Ok(lines) = read_lines("../input/02.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                let policy = Policy::from_str(&ip).unwrap();
                if is_valid(&policy) {
                    count += 1;
                }
            }
        }
        println!("{}", count);
    }
}
