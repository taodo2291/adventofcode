use std::collections::HashSet;
use std::fs;

fn main() {
    let numbers: Vec<isize> = fs::read_to_string("input/1.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let sum = 2020;

    if let Ok((first_num, second_num, third_num)) = find_three_numbers_sums_to_x(numbers, sum) {
        println!(
            "3 numbers that sum to {}:  {}, {} and {}",
            sum, first_num, second_num, third_num
        );
        println!("Mutiply: {}", first_num * second_num * third_num);
    } else {
        println!("Not found");
    }
}

fn find_three_numbers_sums_to_x(numbers: Vec<isize>, x: isize) -> Result<(isize, isize, isize), String> {
    let length = numbers.len();
    for (i, iv) in numbers[0..length - 2].iter().enumerate() {
        let mut set: HashSet<isize> = HashSet::new();
        let sum = x - *iv;
        for jv in &numbers[i + 1..length] {
            let remain_num = sum - *jv as isize;
            if set.contains(&remain_num) {
                return Ok((*iv, *jv, remain_num));
            } else {
                set.insert(*jv);
            }
        }
    }
    Err(String::from("Not found"))
}
