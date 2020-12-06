use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), String> {
    let lines = read_input("input/6.txt")
        .map_err(|e| format!("Could not read the input file due to {}", e.to_string()))?;
    println!("part 1: {}", counts_anyone_answer_yes(&lines));
    println!("part 2: {}", counts_everyone_answer_yes(&lines));

    Ok(())
}

fn read_input(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>())
}

fn counts_everyone_answer_yes(lines: &Vec<String>) -> usize {
    let mut count = 0;
    let mut yes_answers: HashMap<char, u8> = HashMap::new();
    let mut members_count = 0;
    for line in lines {
        if line.trim().is_empty() {
            count += yes_answers
                .values()
                .filter(|v| **v == members_count)
                .count();
            yes_answers.clear();
            members_count = 0;
        } else {
            members_count += 1;
            line.chars().for_each(|c| {
                let current_count: u8 = match yes_answers.get(&c) {
                    Some(n) => *n,
                    None => 0,
                };
                yes_answers.insert(c, current_count + 1);
            });
        }
    }
    count
        + yes_answers
            .values()
            .filter(|v| **v == members_count)
            .count()
}

fn counts_anyone_answer_yes(lines: &Vec<String>) -> usize {
    let mut count = 0;
    let mut yes_answers: HashSet<char> = HashSet::new();
    for line in lines {
        if line.trim().is_empty() {
            count += yes_answers.len();
            yes_answers.clear();
        } else {
            line.chars().for_each(|c| {
                yes_answers.insert(c);
            });
        }
    }
    count + yes_answers.len()
}
