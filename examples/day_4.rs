use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
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
        count_valid_password("input/4.txt", &required_fields, &have_required_fields);
    println!("Part 1: {} valid passports", valid_passport_count);

    println!("=============================");
    let valid_passport_count =
        count_valid_password("input/4.txt", &required_fields, &have_valid_required_fields);
    println!("Part 2: {} valid passports", valid_passport_count);
}
fn have_required_fields(pp: &HashMap<String, String>, required_fields: &[String]) -> bool {
    required_fields.iter().all(|i| pp.contains_key(i))
}

fn count_valid_password(
    path: &str,
    required_fields: &[String],
    f: &dyn Fn(&HashMap<String, String>, &[String]) -> bool,
) -> usize {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|x| x.ok())
        .collect::<Vec<String>>();

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

fn have_valid_required_fields(pp: &HashMap<String, String>, required_fields: &[String]) -> bool {
    if !have_required_fields(pp, required_fields) {
        return false;
    }

    is_valid_birth_year(pp.get("byr").unwrap())
        && is_valid_issue_year(pp.get("iyr").unwrap())
        && is_valid_exprire_year(pp.get("eyr").unwrap())
        && is_valid_height(pp.get("hgt").unwrap())
        && is_valid_hair_color(pp.get("hcl").unwrap())
        && is_valid_eye_color(pp.get("ecl").unwrap())
        && is_valid_pid(pp.get("pid").unwrap())
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
    hcl.get(1..7)
        .unwrap()
        .chars()
        .all(|c| allowed_characters.contains(c))
}

fn is_valid_height(hgt: &str) -> bool {
    let len = hgt.len();
    if len < 3 {
        return false;
    }
    let height = hgt.get(0..len - 2).unwrap().parse::<usize>().unwrap();
    if hgt.ends_with("in") {
        height >= 59 && height <= 76
    } else {
        if hgt.ends_with("cm") {
            height >= 150 && height <= 193
        } else {
            false
        }
    }
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