use regex::Regex;
use std::fs;

pub fn part_one() {
    let input =
        fs::read_to_string("inputs/4_part1.txt").expect("Something went wrong reading the file");

    let mut passport_input: Vec<String> = vec![];
    let mut passport_accumulation = String::new();
    for line in input.lines() {
        if line.is_empty() {
            passport_input.push(passport_accumulation.clone());
            passport_accumulation.clear();
        } else {
            passport_accumulation.push_str(line);
        }
    }

    if !passport_accumulation.is_empty() {
        passport_input.push(passport_accumulation.clone());
    }

    let valid_count = passport_input
        .iter()
        .filter(|x| x.contains("byr"))
        .filter(|x| x.contains("iyr"))
        .filter(|x| x.contains("eyr"))
        .filter(|x| x.contains("hgt"))
        .filter(|x| x.contains("hcl"))
        .filter(|x| x.contains("ecl"))
        .filter(|x| x.contains("pid"))
        //.filter(|x| x.contains("cid"))
        .count();
    println!("Found: {}", valid_count);
}

pub fn part_two() {
    let input =
        fs::read_to_string("inputs/4_part1.txt").expect("Something went wrong reading the file");

    let mut passport_input: Vec<String> = vec![];
    let mut passport_accumulation = String::new();
    for line in input.lines() {
        if line.is_empty() && !passport_accumulation.is_empty() {
            passport_input.push(passport_accumulation.clone());
            passport_accumulation.clear();
        } else {
            passport_accumulation.push_str(" "); //space out serially non-empty lines
            passport_accumulation.push_str(line);
        }
    }

    if !passport_accumulation.is_empty() {
        passport_input.push(passport_accumulation.clone());
    }

    for passport in passport_input.iter() {
        println!("{}", passport);
    }

    println!("Valid Passports");

    let valid: Vec<&String> = passport_input
        .iter()
        .filter(|x| !x.is_empty())
        .filter(|x| validate_birth_year(x))
        .filter(|x| validate_issue_year(x))
        .filter(|x| validate_expiration_year(x))
        .filter(|x| validate_height(x))
        .filter(|x| validate_hair_color(x))
        .filter(|x| validate_eye_color(x))
        .filter(|x| validate_passport_id(x))
        // .filter(|x| x.contains("cid"))
        .collect();

    for valid_passport in valid.iter() {
        println!("{}", valid_passport);
    }

    println!("Found: {}", valid.len());
}

fn validate_height(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^| )hgt:(\d{2,3})(cm|in)(?:$| )").unwrap();
    }

    let height_captures = RE.captures(to_validate);
    if height_captures.is_none() {
        return false;
    }

    let unwrapped = height_captures.unwrap();

    return unwrapped
        .get(1)
        .map(|m| m.as_str())
        .map(|height| height.parse::<u32>().unwrap())
        .map(|height| {
            //gross, surely a better idiomatic way?
            unwrapped
                .get(2)
                .map(|m| m.as_str())
                .map(|unit| {
                    if unit == "cm" {
                        return 150 <= height && height <= 193;
                    }
                    //inches
                    return 59 <= height && height <= 76;
                })
                .unwrap_or(false)
        })
        .unwrap_or(false);
}

fn validate_birth_year(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^| )byr:(\d{4})(?:$| )").unwrap();
    }

    let year_capture = RE.captures(to_validate);
    return year_capture
        .map(|m| m.get(1).unwrap())
        .map(|m| m.as_str())
        .map(|year| year.parse::<u32>().unwrap())
        .map(|year| 1920 <= year && year <= 2002)
        .unwrap_or(false);
}

fn validate_issue_year(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^| )iyr:(\d{4})(?:$| )").unwrap();
    }

    let year_capture = RE.captures(to_validate);
    return year_capture
        .map(|m| m.get(1).unwrap())
        .map(|m| m.as_str())
        .map(|year| year.parse::<u32>().unwrap())
        .map(|year| 2010 <= year && year <= 2020)
        .unwrap_or(false);
}

fn validate_expiration_year(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^| )eyr:(\d{4})(?:$| )").unwrap();
    }

    let year_capture = RE.captures(to_validate);
    return year_capture
        .map(|m| m.get(1).unwrap())
        .map(|m| m.as_str())
        .map(|year| year.parse::<u32>().unwrap())
        .map(|year| 2020 <= year && year <= 2030)
        .unwrap_or(false);
}

fn validate_eye_color(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?:^| )ecl:(amb|blu|brn|gry|grn|hzl|oth)(?:$| )").unwrap();
    }

    return RE.is_match(to_validate);
}

fn validate_passport_id(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^| )pid:\d{9}(?:$| )").unwrap();
    }

    return RE.is_match(to_validate);
}

fn validate_hair_color(to_validate: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^| )hcl:#[0-9a-f]{6}(?:$| )").unwrap();
    }

    return RE.is_match(to_validate);
}
