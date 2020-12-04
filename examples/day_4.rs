use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

macro_rules! map_or_else {
    ($o: expr, $s:expr, $n: expr) => {
        match $o {
            Some(a) => $s(a),
            None => $n,
        }
    };
}

fn main() -> Result<(), String> {
    let lines = match read_input("input/4.txt") {
        Ok(lines) => lines,
        Err(err) => {
            return Err(format!("Could not read the input file due to {}", err.to_string()));
        }
    };
    let required_fields: [String; 7] = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];
    let valid_passport_count =
        count_valid_password(&lines, &required_fields, &have_required_fields);
    println!("Part 1: {} valid passports", valid_passport_count);

    println!("=============================");
    let valid_passport_count =
        count_valid_password(&lines, &required_fields, &have_valid_required_fields);
    println!("Part 2: {} valid passports", valid_passport_count);

    Ok(())
}
fn have_required_fields(pp: &HashMap<String, String>, required_fields: &[String]) -> bool {
    required_fields.iter().all(|i| pp.contains_key(i))
}

fn read_input(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>())
}

fn count_valid_password(
    lines: &Vec<String>,
    required_fields: &[String],
    f: &dyn Fn(&HashMap<String, String>, &[String]) -> bool,
) -> usize {
    let mut passport: HashMap<String, String> = HashMap::new();
    let mut count = 0;
    for line in lines {
        if line.trim().is_empty() {
            if f(&passport, required_fields) {
                count += 1
            }
            passport.retain(|_, _| false);
        } else {
            line.split_whitespace().for_each(|item| {
                let kv = item.split(':').collect::<Vec<&str>>();
                passport.insert(kv[0].to_string(), kv[1].to_string());
            });
        }
    }
    if f(&passport, required_fields) {
        count += 1
    }
    count
}

fn have_valid_required_fields(pp: &HashMap<String, String>, _: &[String]) -> bool {
    map_or_else!(pp.get("byr"), &is_valid_birth_year, false)
        && map_or_else!(pp.get("iyr"), &is_valid_issue_year, false)
        && map_or_else!(pp.get("eyr"), &is_valid_exprire_year, false)
        && map_or_else!(pp.get("hgt"), &is_valid_height, false)
        && map_or_else!(pp.get("hcl"), &is_valid_hair_color, false)
        && map_or_else!(pp.get("ecl"), &is_valid_eye_color, false)
        && map_or_else!(pp.get("pid"), &is_valid_pid, false)
}

fn is_valid_pid(pid: &str) -> bool {
    if pid.len() != 9 {
        return false;
    }

    if let Ok(_) = pid.parse::<usize>() {
        return true;
    }
    false
}

fn is_valid_eye_color(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|i| *i == ecl)
}

fn is_valid_hair_color(hcl: &str) -> bool {
    if hcl.len() != 7 || !hcl.starts_with("#") {
        return false;
    }
    let allowed_characters = "0123456789abcdef";
    hcl.chars().skip(1).all(|c| allowed_characters.contains(c))
}

fn is_valid_height(hgt: &str) -> bool {
    if let Some(height) = hgt.get(0..hgt.len() - 2) {
        if let Ok(height) = height.parse::<usize>() {
            if hgt.ends_with("in") {
                return height >= 59 && height <= 76;
            } else {
                if hgt.ends_with("cm") {
                    return height >= 150 && height <= 193;
                }
            }
        }
    }
    false
}

fn is_valid_exprire_year(eyr: &str) -> bool {
    if let Ok(year) = eyr.parse::<usize>() {
        year >= 2020 && year <= 2030
    } else {
        false
    }
}

fn is_valid_issue_year(iyr: &str) -> bool {
    if let Ok(issue_year) = iyr.parse::<usize>() {
        issue_year >= 2010 && issue_year <= 2020
    } else {
        false
    }
}

fn is_valid_birth_year(byr: &str) -> bool {
    if let Ok(birth_year) = byr.parse::<usize>() {
        birth_year >= 1920 && birth_year <= 2002
    } else {
        false
    }
}
