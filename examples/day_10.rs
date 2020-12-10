use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut input = read_input("input/10.txt").expect("Could not read input file!!");
    input.push(0);
    input.sort();

    println!(
        "Part 1: {}",
        find_multiply_one_three_differences_jolt_count(&input)
    );
    println!("Part 2: {}", count_distinct_ways(&input));
}

fn find_multiply_one_three_differences_jolt_count(input: &Vec<usize>) -> usize {
    let mut one_jolt_differences_count: usize = 0;
    let mut three_jolts_differences_count: usize = 1;
    input.iter().fold(0, |acc, x| {
        let delta = *x - acc;
        if delta == 1 {
            one_jolt_differences_count += 1;
        } else {
            if delta == 3 {
                three_jolts_differences_count += 1;
            }
        }
        *x
    });
    one_jolt_differences_count * three_jolts_differences_count
}

fn count_distinct_ways(input: &Vec<usize>) -> usize {
    let min = input[0];
    let mut distinct_ways_count_map: HashMap<usize, usize> = HashMap::new();
    for (i, val) in input.iter().enumerate() {
        if *val == min {
            distinct_ways_count_map.insert(*val, 1);
            continue;
        }
        let range = if i <= 2 { i } else { 3 };
        let count = input[i - range..i]
            .iter()
            .filter(|n| val - *n <= 3)
            .fold(0, |acc, n| acc + distinct_ways_count_map.get(n).unwrap());
        distinct_ways_count_map.insert(*val, count);
    }
    *distinct_ways_count_map
        .get(&input[input.len() - 1])
        .unwrap()
}

fn read_input(path: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(path)?)
        .lines()
        .flat_map(|l| l.ok())
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>();
    Ok(input?)
}
