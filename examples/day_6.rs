use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    let lines = read_input("input/6.txt").unwrap();
    println!("part 1: {}", counts(&lines));
}

fn read_input(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>())
}

fn counts(lines: &Vec<String>) -> usize {
    let mut count = 0;
    let mut yes_answers:HashSet<char> = HashSet::new();
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