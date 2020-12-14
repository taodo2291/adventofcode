use std::fs;
fn main() {
    let input = fs::read_to_string("input/13.txt").unwrap();
    let (earliest_timestamp, buses) = read_input(&input);
    println!("Part 1: {}", part_1(earliest_timestamp, buses));

    let (earliest_timestamp, buses) = read_input_2(&input);
    println!("Part 2: {}", part_2(earliest_timestamp, &buses));
}

fn part_1(earliest_timestamp: usize, buses: Vec<usize>) -> usize {
    for i in earliest_timestamp.. {
        for b in &buses {
            if i % b == 0 {
                return b * (i - earliest_timestamp);
            }
        }
    }
    panic!("not found");
}

fn part_2(_: usize, buses: &Vec<(usize, usize)>) -> usize {
    let step: usize = buses[0].0;

    let mut mins = 100000000000000 - 100000000000000 % step;
    loop {
        if buses[1..].iter().all(|(id, time)| (mins + time) % id == 0) {
            return mins;
        }
        mins += step;
    }
}
fn read_input(content: &String) -> (usize, Vec<usize>) {
    let lines = content.lines().collect::<Vec<&str>>();
    let bus_ids: Vec<usize> = lines[1]
        .split(',')
        .filter(|b| *b != "x")
        .map(|b| b.parse::<usize>().unwrap())
        .collect();
    (lines[0].parse::<usize>().unwrap(), bus_ids)
}

fn read_input_2(content: &String) -> (usize, Vec<(usize, usize)>) {
    let mut vec: Vec<(usize, usize)> = Vec::new();
    let lines = content.lines().collect::<Vec<&str>>();
    for (index, id) in lines[1].split(',').enumerate() {
        if id != "x" {
            vec.push((id.parse::<usize>().unwrap(), index));
        }
    }
    (lines[0].parse::<usize>().unwrap(), vec)
}

