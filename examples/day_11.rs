use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let mut input = read_input("input/11.txt");
    assert!(!input.is_empty());
    let row_len = input.len();
    let col_len = input[0].len();

    let mut input_part_1 = input.clone();
    println!("Part 1: {}", part1(row_len, col_len, &mut input_part_1));
    println!("Part 2: {}", part2(row_len, col_len, &mut input));
}
#[derive(Debug, PartialEq, Clone)]
enum SeatState {
    Floor,
    Occupied,
    Empty,
}

fn read_input(path: &str) -> Vec<Vec<SeatState>> {
    let mut seat_state: Vec<Vec<SeatState>> = Vec::new();
    let lines = BufReader::new(File::open(path).unwrap()).lines();
    for line in lines {
        if let Ok(line) = line {
            let row = line
                .chars()
                .map(|c| match c {
                    '#' => SeatState::Occupied,
                    '.' => SeatState::Floor,
                    _ => SeatState::Empty,
                })
                .collect::<Vec<SeatState>>();
            seat_state.push(row);
        }
    }
    seat_state
}

fn part1(row_num: usize, column_num: usize, map: &mut Vec<Vec<SeatState>>) -> usize {
    loop {
        let mut changes: Vec<(usize, usize)> = Vec::new();
        for r in 0..row_num {
            for c in 0..column_num {
                let seat_state = &map[r][c];
                if *seat_state == SeatState::Floor {
                    continue;
                }
                let adjency_count = count_adjency_seat(r as isize, c as isize, &map);

                if *seat_state == SeatState::Empty {
                    if adjency_count == 0 {
                        changes.push((r, c));
                    }
                } else {
                    if adjency_count >= 4 {
                        changes.push((r, c));
                    }
                }
            }
        }
        if changes.is_empty() {
            break;
        }
        for (r, c) in changes {
            let current = &map[r][c];
            match current {
                SeatState::Empty => map[r][c] = SeatState::Occupied,
                SeatState::Occupied => map[r][c] = SeatState::Empty,
                _ => {}
            }
        }
    }
    map.iter()
        .flatten()
        .filter(|s| **s == SeatState::Occupied)
        .count()
}

fn count_adjency_seat(row: isize, col: isize, map: &Vec<Vec<SeatState>>) -> usize {
    let mut count = 0;
    let max_row = map.len() as isize - 1;
    let max_col = map[0].len() as isize - 1;
    for r in row - 1..row + 2 {
        if r < 0 || r > max_row {
            continue;
        }
        for c in col - 1..col + 2 {
            if c < 0 || c > max_col {
                continue;
            }
            if r == row && c == col {
                continue;
            }
            if map[r as usize][c as usize] == SeatState::Occupied {
                count += 1;
            }
        }
    }
    count
}

fn part2(row_len: usize, col_len: usize, map: &mut Vec<Vec<SeatState>>) -> usize {
    loop {
        let mut changes: Vec<(usize, usize)> = Vec::new();
        for r in 0..row_len {
            for c in 0..col_len {
                let seat_state = &map[r][c];
                if *seat_state == SeatState::Floor {
                    continue;
                }
                let adjency_count = count_seat_in_sign(r, c, &map, row_len, col_len);

                if *seat_state == SeatState::Empty {
                    if adjency_count == 0 {
                        changes.push((r, c));
                    }
                } else {
                    if adjency_count >= 5 {
                        changes.push((r, c));
                    }
                }
            }
        }
        if changes.is_empty() {
            break;
        }
        for (r, c) in changes {
            let current = &map[r][c];
            match current {
                SeatState::Empty => map[r][c] = SeatState::Occupied,
                SeatState::Occupied => map[r][c] = SeatState::Empty,
                _ => {}
            }
        }
    }
    map.iter()
        .flatten()
        .filter(|s| **s == SeatState::Occupied)
        .count()
}

fn count_seat_in_sign(
    row: usize,
    col: usize,
    map: &Vec<Vec<SeatState>>,
    row_len: usize,
    col_len: usize,
) -> usize {
    let mut count = 0;

    let mut update_count = |s: &SeatState| match s {
        SeatState::Occupied => {
            count += 1;
            true
        }
        SeatState::Empty => true,
        _ => false,
    };
    //first left
    for i in 1..col + 1 {
        if update_count(&map[row][col - i]) {
            break;
        }
    }
    //first right'
    for i in col + 1..col_len {
        if update_count(&map[row][i]) {
            break;
        }
    }
    //first top
    for i in 1..row + 1 {
        if update_count(&map[row - i][col]) {
            break;
        }
    }
    //first bottom
    for i in row + 1..row_len {
        if update_count(&map[i][col]) {
            break;
        }
    }
    //first left-top
    let min = cmp::min(row, col);
    for i in 1..min + 1 {
        if update_count(&map[row - i][col - i]) {
            break;
        }
    }
    //first right-top
    let min = cmp::min(row + 1, col_len - col);
    for i in 1..min {
        if update_count(&map[row - i][col + i]) {
            break;
        }
    }
    //first left-bottom
    let min = cmp::min(row_len - row, col + 1);
    for i in 1..min {
        if update_count(&map[row + i][col - i]) {
            break;
        }
    }

    //first right-bottom
    let min = cmp::min(row_len - row, col_len - col);
    for i in 1..min {
        if update_count(&map[row + i][col + i]) {
            break;
        }
    }
    count
}