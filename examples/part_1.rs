use std::collections::HashSet;
use std::fs;

fn main() {
    let numbers = fs::read_to_string("input/1.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let sum = 2020;
    if let Ok((first_num, second_num)) = find_two_numbers_sums_to_x(numbers, sum) {
        println!(
            "2 numbers that sum to {}: {} and {}",
            sum, first_num, second_num
        );
        println!("multiply: {}", first_num * second_num);
    } else {
        println!("Not found");
    }
}

fn find_two_numbers_sums_to_x(numbers: Vec<isize>, x: isize) -> Result<(isize, isize), String> {
    let mut set: HashSet<isize> = HashSet::new();
    for num in numbers {
        let remain_number = x - num;
        if set.contains(&remain_number) {
            return Ok((num, remain_number));
        } else {
            set.insert(num);
        }
    }
    Err(String::from("Not found"))
}
