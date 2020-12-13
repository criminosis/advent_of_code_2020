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
