use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let instructions = read_input("input/12.txt");
    println!("Part 1: {}", part_1(&instructions));
    println!("Part 2: {}", part_2(&instructions));
}

fn read_input(path: &str) -> Vec<Instruction> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .filter_map(|l| l.ok())
        .map(Instruction::from)
        .collect()
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut ship = Ship {
        position: (0, 0),
        waypoint: (1, 0),
    };
    for instruction in instructions {
        match instruction {
            Instruction::Left(angle) => {
                let mut angle = *angle;
                while angle >= 90 {
                    angle -= 90;
                    ship.waypoint = (ship.waypoint.1, -ship.waypoint.0);
                }
            }
            Instruction::Right(angle) => {
                let mut angle = *angle;
                while angle >= 90 {
                    angle -= 90;
                    ship.waypoint = (-ship.waypoint.1, ship.waypoint.0);
                }
            }
            Instruction::North(s) => ship.move_forward((0, -s)),
            Instruction::South(s) => ship.move_forward((0, *s)),
            Instruction::West(s) => ship.move_forward((-s, 0)),
            Instruction::Est(s) => ship.move_forward((*s, 0)),
            Instruction::Forward(s) => {
                ship.move_forward((*s * ship.waypoint.0, *s * ship.waypoint.1))
            }
        }
    }
    ship.manhattan_distance()
}

fn part_2(instructions: &Vec<Instruction>) -> usize {
    let mut ship = Ship {
        position: (0, 0),
        waypoint: (10, -1),
    };
    for instruction in instructions {
        match instruction {
            Instruction::Left(angle) => {
                let mut angle = *angle;
                while angle >= 90 {
                    angle -= 90;
                    ship.waypoint = (ship.waypoint.1, -ship.waypoint.0);
                }
            }
            Instruction::Right(angle) => {
                let mut angle = *angle;
                while angle >= 90 {
                    angle -= 90;
                    ship.waypoint = (-ship.waypoint.1, ship.waypoint.0);
                }
            }
            Instruction::North(s) => ship.waypoint.1 -= s,
            Instruction::South(s) => ship.waypoint.1 += s,
            Instruction::West(s) => ship.waypoint.0 -= s,
            Instruction::Est(s) => ship.waypoint.0 += s,
            Instruction::Forward(s) => {
                ship.move_forward((*s * ship.waypoint.0, *s * ship.waypoint.1))
            }
        }
    }
    ship.manhattan_distance()
}

struct Ship {
    position: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship {
    fn manhattan_distance(&self) -> usize {
        (self.position.0.abs() + self.position.1.abs()) as usize
    }

    fn move_forward(&mut self, delta: (i32, i32)) {
        self.position = (self.position.0 + delta.0, self.position.1 + delta.1)
    }
}

enum Instruction {
    Left(i32),
    Right(i32),
    North(i32),
    South(i32),
    West(i32),
    Est(i32),
    Forward(i32),
}

impl From<String> for Instruction {
    fn from(raw: String) -> Self {
        let items = raw.split_at(1);
        let val = items.1.parse::<i32>().expect("invalid format");
        match items.0 {
            "L" => Instruction::Left(val),
            "R" => Instruction::Right(val),
            "N" => Instruction::North(val),
            "S" => Instruction::South(val),
            "W" => Instruction::West(val),
            "E" => Instruction::Est(val),
            _ => Instruction::Forward(val),
        }
    }
}
