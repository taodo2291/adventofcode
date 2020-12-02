use std::io::{prelude::*, BufReader};
use std::{fs::File, str::FromStr};

fn main() {

    let match_count_policy_check = |line: &str| {
        let pw: Password<MatchCountPolicy> = Password::from_str(line).unwrap();
        return pw.is_valid();
    };
    println!(
        "part 1: {} valid passwords\r\n",
        count_valid_password_in_file("input/2.txt", match_count_policy_check)
    );

    let match_position_policy_check = |line: &str| {
        let pw: Password<PositionPolicy> = Password::from_str(line).unwrap();
        return pw.is_valid();
    };
    println!(
        "part 2: {} valid passwords",
        count_valid_password_in_file("input/2.txt", match_position_policy_check)
    );
}

fn count_valid_password_in_file(path: &str, f: fn(&str) -> bool) -> usize {
    let mut count = 0;
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if f(&line) {
                count += 1;
            }
        }
    }
    count
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
            .trim()
            .split('-')
            .map(|e| e.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        Ok(MatchCountPolicy {
            pattern,
            min_occurs: range_elements[0],
            max_occurs: range_elements[1],
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
        assert_eq!(elements.len(), 2);
        let pattern = elements[1].trim().chars().collect::<Vec<char>>();
        assert_eq!(pattern.len(), 1);
        let pattern = pattern[0];

        let range_elements = elements[0]
            .trim()
            .split('-')
            .map(|e| e.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        Ok(PositionPolicy {
            pattern,
            position_1: range_elements[0],
            position_2: range_elements[1],
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
        let mut match_count = 0;
        if chars[self.policy.position_1 - 1] == self.policy.pattern {
            match_count += 1;
        }
        if chars[self.policy.position_2 - 1] == self.policy.pattern {
            match_count += 1;
        }

        if match_count != 1 {
            return false;
        }
        true
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