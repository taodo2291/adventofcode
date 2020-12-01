use std::collections::HashSet;
use std::fs;

fn main() {
    let numbers = fs::read_to_string("input/1.txt")
        .expect("input file must be exist")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let sum = 2020;
    println!("==============PART-1============");
    resolve_part_1(&numbers, sum);

    println!("\n");
    
    println!("==============PART-2============");
    resolve_part_2(&numbers, sum);
}

fn resolve_part_1(numbers: &Vec<isize>, sum: isize) {
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

fn resolve_part_2(numbers: &Vec<isize>, sum: isize) {
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

fn find_two_numbers_sums_to_x(numbers: &Vec<isize>, x: isize) -> Result<(isize, isize), String> {
    let mut set: HashSet<isize> = HashSet::new();
    for num in numbers {
        let remain_number = x - num;
        if set.contains(&remain_number) {
            return Ok((*num, remain_number));
        } else {
            set.insert(*num);
        }
    }
    Err(String::from("Not found"))
}

fn find_three_numbers_sums_to_x(
    numbers: &Vec<isize>,
    x: isize,
) -> Result<(isize, isize, isize), String> {
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
