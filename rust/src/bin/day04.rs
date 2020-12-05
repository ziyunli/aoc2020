extern crate aoc2020;

use aoc2020::common::read_lines;
use regex::Regex;
use std::collections::HashMap;

fn is_valid(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn is_valid_range(year: usize, lo: usize, hi: usize) -> bool {
    year >= lo && year <= hi
}

fn is_valid_height(text: &str) -> bool {
    let re = Regex::new(r"(?P<val>\d+)(?P<measure>(cm|in))").unwrap();
    match re.captures(text) {
        Some(caps) => {
            let val = caps["val"].parse::<usize>().unwrap();
            if caps["measure"].to_string() == "cm" {
                if val < 150 || val > 193 {
                    return false;
                }
            } else {
                if val < 59 || val > 76 {
                    return false;
                }
            }
            true
        }
        _ => return false,
    }
}

#[test]
fn test_is_valid_height() {
    assert!(is_valid_height("60in"));
    assert!(is_valid_height("190cm"));
    assert!(!is_valid_height("190in"));
    assert!(!is_valid_height("190"));
}

fn is_valid_color(text: &str) -> bool {
    let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
    re.is_match(text)
}

#[test]
fn test_is_valid_color() {
    assert!(is_valid_color("#123abc"));
    assert!(!is_valid_color("#123abz"));
    assert!(!is_valid_color("123abc"));
}

fn is_valid_eye_color(text: &str) -> bool {
    let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    re.is_match(text)
}

#[test]
fn test_is_valid_eye_color() {
    assert!(is_valid_eye_color("brn"));
    assert!(!is_valid_eye_color("wat"));
}

fn is_valid_passport_id(text: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(text)
}

#[test]
fn test_is_valid_passport_id() {
    assert!(is_valid_passport_id("000000001"));
    assert!(!is_valid_passport_id("0123456789"));
}

fn is_valid_improved(passport: &HashMap<String, String>) -> bool {
    if !is_valid(&passport) {
        return false;
    }

    let byr = passport.get("byr").unwrap().parse::<usize>().unwrap();
    if !is_valid_range(byr, 1920, 2002) {
        return false;
    }

    let iyr = passport.get("iyr").unwrap().parse::<usize>().unwrap();
    if !is_valid_range(iyr, 2010, 2020) {
        return false;
    }

    let eyr = passport.get("eyr").unwrap().parse::<usize>().unwrap();
    if !is_valid_range(eyr, 2020, 2030) {
        return false;
    }

    let hgt = passport.get("hgt").unwrap();
    if !is_valid_height(hgt) {
        return false;
    }

    let hcl = passport.get("hcl").unwrap();
    if !is_valid_color(hcl) {
        return false;
    }

    let ecl = passport.get("ecl").unwrap();
    if !is_valid_eye_color(ecl) {
        return false;
    }

    let pid = passport.get("pid").unwrap();
    if !is_valid_passport_id(pid) {
        return false;
    }

    true
}

fn update_password(
    line: &String,
    mut passport: HashMap<String, String>,
) -> HashMap<String, String> {
    let pairs = line.trim().split_whitespace();
    for pair in pairs {
        let tokens = pair.split(':').collect::<Vec<_>>();
        passport.insert(tokens[0].to_string(), tokens[1].to_string());
    }
    passport
}

fn main() {
    if let Ok(lines) = read_lines("../input/04.txt") {
        let mut passport = HashMap::new();
        let mut count = 0;
        let mut better_count = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim() == "" {
                    if is_valid(&passport) {
                        count += 1
                    }
                    if is_valid_improved(&passport) {
                        better_count += 1
                    }
                    passport = HashMap::new();
                } else {
                    passport = update_password(&ip, passport);
                }
            }
        }
        println!("Part 1 valid passports: {}", count);
        println!("Part 2 valid passports: {}", better_count);
    }
}
