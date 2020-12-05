extern crate aoc2020;

use aoc2020::common::read_lines;
use std::collections::HashMap;
use regex::Regex;


fn is_valid(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr") &&
        passport.contains_key("iyr") &&
        passport.contains_key("eyr") &&
        passport.contains_key("hgt") &&
        passport.contains_key("hcl") &&
        passport.contains_key("ecl") &&
        passport.contains_key("pid")
}

fn is_valid_byr(byr: usize) -> bool {
    byr >= 1920 && byr <= 2002
}

#[test]
fn test_is_valid_byr() {
    assert!(is_valid_byr(2002));
    assert!(!is_valid_byr(2003));
}

fn is_valid_improved(passport: &HashMap<String, String>) -> bool {
    if !is_valid(&passport) {
        return false;
    }

    let byr = passport.get("byr").unwrap().parse::<usize>().unwrap();
    if !is_valid_byr(byr) {
        return false;
    }

    let iyr = passport.get("iyr").unwrap().parse::<usize>().unwrap();
    if iyr > 2020 || iyr < 2010 {
        return false;
    }

    let eyr = passport.get("eyr").unwrap().parse::<usize>().unwrap();
    if eyr > 2030 || eyr < 2020 {
        return false;
    }

    let hgt = passport.get("hgt").unwrap();
    let re = Regex::new(r"(?P<val>\d+)(?P<measure>[cm|in])").unwrap();
    match re.captures(hgt) {
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
        }
        _ => return false
    }

    let hcl = passport.get("hcl").unwrap();
    let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
    if !re.is_match(hcl) {
        return false;
    }

    let ecl = passport.get("ecl").unwrap();
    let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    if !re.is_match(ecl) {
        return false;
    }

    let pid = passport.get("pid").unwrap();
    let re = Regex::new(r"[0-9]{9}").unwrap();
    if re.is_match(pid) {
        return false;
    }

    true
}

fn update_password(line: &String, mut passport: HashMap<String, String>) -> HashMap<String, String>{
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
