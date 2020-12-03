use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    //read the file
    let maps = read_input("input/3.txt").unwrap();

    // ============PART 1=================
    // Right 3, down 1.
    println!("Part 1: {} trees", count_tree(&maps, 1, 3));

    // ============PART 2=================
    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mutiply = slopes
        .iter()
        .map(|slope| count_tree(&maps, slope.1, slope.0))
        .fold(1, |acc, x| acc * x);

    println!("Part 2: {}", mutiply);
}

fn count_tree(maps: &Vec<Vec<bool>>, down: usize, right: usize) -> usize {
    let mut col = 0;
    let mut row = 0;
    let mut tree_count = 0;
    let map_width = maps[0].len();

    while row < maps.len() {
        if maps[row][col] {
            tree_count += 1;
        }
        row += down;
        col += right;
        if col >= map_width {
            col -= map_width as usize;
        }
    }
    tree_count
}

fn read_input(path: &str) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .map(|l| {
            l.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>())
}
