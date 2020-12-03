extern crate aoc2020;

use aoc2020::common::read_lines;

use std::collections::HashSet;

fn main() {
    if let Ok(lines) = read_lines("../input/01.txt") {
        let nums = lines
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let length = nums.len();

        for i in 0..length - 1 {
            let mut found: HashSet<i32> = HashSet::new();
            let a = nums[i];
            let rem = 2020 - a;
            for j in i + 1..length {
                let b = nums[j];
                let c = rem - b;
                if found.contains(&c) {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    return;
                }
                found.insert(b);
            }
        }
    }
}
