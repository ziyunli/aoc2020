use std::str::FromStr;

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
        let dash = line.find('-').unwrap();
        let whitespace = line.find(' ').unwrap();
        let semicolon = line.find(':').unwrap();

        let lo = line[0..dash].parse::<usize>().unwrap();
        let hi = line[dash + 1..whitespace].parse::<usize>().unwrap();
        let char = line.chars().nth(whitespace + 1).unwrap();
        let password = String::from_str(&line[semicolon + 2..]).unwrap();

        Ok(Policy { char, lo, hi, password })
    }
}
