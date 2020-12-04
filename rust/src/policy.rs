use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Policy {
    pub char: char,
    pub lo: usize,
    pub hi: usize,
    pub password: String,
}

impl FromStr for Policy {
    type Err = std::string::ParseError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"(?P<lo>\d+)-(?P<hi>\d+) (?P<char>[a-z]): (?P<password>[a-z]+)").unwrap();
        let caps = re.captures(line).unwrap();
        return Ok(Policy {
            lo: caps["lo"].parse::<usize>().unwrap(),
            hi: caps["hi"].parse::<usize>().unwrap(),
            char: caps["char"].chars().next().unwrap(),
            password: String::from_str(&caps["password"]).unwrap(),
        });
    }
}
