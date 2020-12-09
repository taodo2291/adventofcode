use std::cmp;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input =
        read_input("input/9.txt").map_err(|msg| format!("Could not read input due to {}", msg))?;

    let invalid_xmas = find_invalid_xmas(&input).ok_or("Could not found invalid xmas")?;
    println!("Part 1: {}", invalid_xmas);
    println!(
        "Part 2: {}",
        find_encryption_weakness(invalid_xmas, &input)
            .ok_or("Could not found encryption weakness")?
    );
    Ok(())
}

fn read_input(path: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open(path)?)
        .lines()
        .flat_map(|l| l.ok())
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>();
    Ok(input?)
}

fn find_invalid_xmas(input: &Vec<usize>) -> Option<usize> {
    input.windows(26).find_map(|nums| {
        let target = nums[nums.len() - 1];
        for (i, first_num) in nums[..24].iter().enumerate() {
            for second_num in &nums[i + 1..25] {
                if first_num + second_num == target {
                    return None;
                }
            }
        }
        return Some(target);
    })
}

fn find_encryption_weakness(invalid_xmas: usize, numbers: &Vec<usize>) -> Option<usize> {
    for range in 2..numbers.len() - 1 {
        for (index, num) in numbers[..numbers.len() - range].iter().enumerate() {
            if *num > invalid_xmas {
                break;
            }
            let sum = numbers[index..index + range].iter().sum::<usize>();
            if sum == invalid_xmas {
                let min_max = numbers[index..index + range]
                    .iter()
                    .fold((usize::MAX, usize::MIN), |(min, max), x| {
                        (cmp::min(min, *x), cmp::max(max, *x))
                    });
                return Some(min_max.0 + min_max.1);
            }
        }
    }
    None
}
