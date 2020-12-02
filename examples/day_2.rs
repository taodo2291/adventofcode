use std::error::Error;
use std::io::{prelude::*, BufReader};
use std::{fs::File, str::FromStr};

fn main() {
    let match_count_policy_check = |line: &str| {
        if let Ok(pw) = Password::<MatchCountPolicy>::from_str(line) {
            pw.is_valid()
        } else {
            false
        }
    };

    match count_valid_password_in_file("input/2.txt", match_count_policy_check) {
        Ok(valid_password_count) => {
            println!("part 1: {} valid passwords\r\n", valid_password_count)
        }
        Err(err) => println!("part 1: fail due to {}\r\n", err.to_string()),
    }

    let match_position_policy_check = |line: &str| {
        if let Ok(pw) = Password::<PositionPolicy>::from_str(line) {
            pw.is_valid()
        } else {
            false
        }
    };
    match count_valid_password_in_file("input/2.txt", match_position_policy_check) {
        Ok(valid_password_count) => {
            println!("part 1: {} valid passwords", valid_password_count)
        }
        Err(err) => println!("part 2: fail due to {}", err.to_string()),
    }
}

fn count_valid_password_in_file(
    path: &str,
    is_valid: fn(&str) -> bool,
) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .filter(|l| is_valid(&l))
        .count())
}

#[derive(Debug)]
struct MatchCountPolicy {
    pattern: char,
    min_occurs: usize,
    max_occurs: usize,
}

impl FromStr for MatchCountPolicy {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let elements = raw
            .split_whitespace()
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        if elements.len() != 2 {
            return Err(String::from("Invalid input"));
        }
        let pattern = elements[1].trim().chars().collect::<Vec<char>>();
        if pattern.len() != 1 {
            return Err(String::from("Invalid input"));
        }

        let range_elements = elements[0]
            .split('-')
            .map(|e| e.trim())
            .collect::<Vec<&str>>();

        let min_occurs: usize = match range_elements[0].parse() {
            Ok(num) => num,
            Err(err) => return Err(err.to_string()),
        };

        let max_occurs: usize = match range_elements[1].parse() {
            Ok(num) => num,
            Err(err) => return Err(err.to_string()),
        };

        Ok(MatchCountPolicy {
            pattern: pattern[0],
            min_occurs,
            max_occurs,
        })
    }
}

#[derive(Debug)]
struct PositionPolicy {
    pattern: char,
    position_1: usize,
    position_2: usize,
}

impl FromStr for PositionPolicy {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let elements = raw.split_whitespace().into_iter().collect::<Vec<&str>>();
        if elements.len() != 2 {
            return Err(String::from("Invalid input"));
        }
        let pattern = elements[1].trim().chars().collect::<Vec<char>>();
        if pattern.len() != 1 {
            return Err(String::from("Invalid input"));
        }
        let pattern = pattern[0];

        let range_elements = elements[0]
            .split('-')
            .map(|e| e.trim())
            .collect::<Vec<&str>>();

        let position_1: usize = match range_elements[0].parse() {
            Ok(num) => num,
            Err(err) => return Err(err.to_string()),
        };

        let position_2: usize = match range_elements[1].parse() {
            Ok(num) => num,
            Err(err) => return Err(err.to_string()),
        };

        Ok(PositionPolicy {
            pattern,
            position_1,
            position_2,
        })
    }
}

#[derive(Debug)]
struct Password<T: FromStr> {
    policy: T,
    password_text: String,
}

impl FromStr for Password<PositionPolicy> {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let elements = raw.split(':').collect::<Vec<&str>>();
        assert_eq!(elements.len(), 2);

        let policy = PositionPolicy::from_str(elements[0].trim())?;
        Ok(Password {
            password_text: elements[1].trim().to_string(),
            policy,
        })
    }
}

impl ValidCheck for Password<PositionPolicy> {
    fn is_valid(&self) -> bool {
        let chars = self.password_text.chars().collect::<Vec<char>>();
        (chars[self.policy.position_1 - 1] == self.policy.pattern)
            != (chars[self.policy.position_2 - 1] == self.policy.pattern)
    }
}

impl FromStr for Password<MatchCountPolicy> {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let elements = raw.split(':').collect::<Vec<&str>>();
        if elements.len() != 2 {
            return Err(String::from("Invalid input"));
        }

        let policy = MatchCountPolicy::from_str(elements[0].trim())?;
        Ok(Password {
            password_text: elements[1].trim().to_string(),
            policy,
        })
    }
}

impl ValidCheck for Password<MatchCountPolicy> {
    fn is_valid(&self) -> bool {
        let time_occurs = self.password_text.matches(self.policy.pattern).count();
        if time_occurs < self.policy.min_occurs || time_occurs > self.policy.max_occurs {
            return false;
        }
        true
    }
}

trait ValidCheck {
    fn is_valid(&self) -> bool;
}