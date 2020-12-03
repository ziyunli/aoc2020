extern crate aoc2020;
use aoc2020::common::read_lines;

use std::collections::HashSet;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../input/01.txt") {
        let mut found: HashSet<i32> = HashSet::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let n = ip.parse::<i32>().unwrap();
                let m = 2020 - n;
                if found.contains(&m) {
                    println!("{} * {} = {}", n, m, n * m);
                    return;
                }
                found.insert(n);
            }
        }
    }
}
