use std::io::{prelude::*, BufReader};
use std::{collections::HashSet, fs::File};

fn main() {
    let lines = read_input("input/8.txt").unwrap();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

enum Operation {
    Acc(isize),
    Nop(isize),
    Jmp(isize),
}

impl From<String> for Operation {
    fn from(text: String) -> Self {
        let items = text.split_whitespace().collect::<Vec<&str>>();
        let val = items[1].parse::<isize>().unwrap();
        match items[0] {
            "acc" => Operation::Acc(val),
            "jmp" => Operation::Jmp(val),
            "nop" => Operation::Nop(val),
            _ => panic!("invalid operation"),
        }
    }
}

fn part2(operations: &Vec<Operation>) -> isize {
    let max_step = operations.len() - 1;
    let mut change_index = 0;

    let mut index: usize = 0;
    let mut _operation = Operation::Nop(0);
    for op in operations {
        index += 1;
        if index - 1 <= change_index {
            continue;
        }
        match op {
            Operation::Jmp(num) => {
                change_index = index - 1;
                _operation = Operation::Nop(*num);
            }
            Operation::Nop(num) => {
                change_index = index - 1;
                _operation = Operation::Jmp(*num);
            }
            _ => continue,
        }
        let (acc, last_step) = run_with_change_one_step(&operations, _operation, change_index);

        if last_step > max_step {
            return acc;
        }
    }

    0
}

fn run_with_change_one_step(
    operations: &Vec<Operation>,
    change_operate: Operation,
    changed_index: usize,
) -> (isize, usize) {
    let mut acc = 0;
    let mut step: usize = 0;
    let max_step = operations.len() - 1;
    let mut executed_op: HashSet<usize> = HashSet::new();
    loop {
        if step > max_step || executed_op.contains(&step) {
            break;
        }
        executed_op.insert(step);
        let operation = if step == changed_index {
            &change_operate
        } else {
            &operations[step]
        };
        match operation {
            Operation::Acc(num) => acc += num,
            Operation::Jmp(num) => {
                let a = (step as isize + num) as usize;
                step = a;
                continue;
            }
            Operation::Nop(_) => {}
        }
        step += 1;
    }

    (acc, step)
}

fn part1(lines: &Vec<Operation>) -> isize {
    let mut acc = 0;
    let mut step: usize = 0;
    let max_step = lines.len() - 1;
    let mut executed_op: HashSet<usize> = HashSet::new();
    loop {
        if step > max_step || executed_op.contains(&step) {
            break;
        }
        executed_op.insert(step);
        match &lines[step] {
            Operation::Acc(num) => acc += num,
            Operation::Jmp(num) => {
                let a = (step as isize + num) as usize;
                step = a;
                continue;
            }
            Operation::Nop(_) => {}
        }
        step += 1;
    }

    acc
}

fn read_input(path: &str) -> Result<Vec<Operation>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|l| Operation::from(l))
        .collect::<Vec<Operation>>())
}