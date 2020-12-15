use std::fs;
use regex::Regex;

pub fn part_one() {
    let valid_count = fs::read_to_string("inputs/2_part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter(|x| validate_part_1(x))
        .count();
        println!("Valid: {}", valid_count);

    // validate("1-3 z: vzzzwz");
}

pub fn part_two() {
    let valid_count = fs::read_to_string("inputs/2_part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter(|x| validate_part_2(x))
        .count();
        println!("Valid: {}", valid_count);
}


fn validate_part_1(input_row: &str) -> bool {
//6-16 c: ccccczcccccccccgcc
//X-Y means there must be at least 6 but no more than 16 of the following character, and then the input value
//we should try to not re-create this regex each time?
let re = Regex::new(r"^(\d+)-(\d+) (\D): (.+)$").unwrap();
let captures = re.captures(input_row).unwrap();

let min_limit = &captures[1].parse::<usize>().unwrap();
let max_limit = &captures[2].parse::<usize>().unwrap();
let character = &captures[3].chars().nth(0).unwrap();
let value = &captures[4];

let number_of_characters_in_value = value.chars().filter(|x| x == character).count();
let is_valid = number_of_characters_in_value >= *min_limit && number_of_characters_in_value <= *max_limit;
// let regex_format = format!("{}{{{},{}}}", character, min_limit, max_limit);
// let validation = Regex::new(&regex_format).unwrap();
// let is_valid = validation.is_match(value);
println!("Got {}-{} testing {} in {}. Valid? {}", min_limit, max_limit, character, value, is_valid);
return is_valid;
}


fn validate_part_2(input_row: &str) -> bool {
    //6-16 c: ccccczcccccccccgcc
    //X-Y means to check if either position (not index) 6 or 16 is c
    //we should try to not re-create this regex each time?
    let re = Regex::new(r"^(\d+)-(\d+) (\D): (.+)$").unwrap();
    let captures = re.captures(input_row).unwrap();
    
    let first_index = &captures[1].parse::<usize>().unwrap() - 1;
    let second_index = &captures[2].parse::<usize>().unwrap() - 1;
    let character = captures[3].chars().nth(0).unwrap();
    let value = &captures[4];
    
    let first_index_match = value.chars().nth(first_index).unwrap() == character;
    let second_index_match = value.chars().nth(second_index).unwrap() == character;
    return first_index_match ^ second_index_match;
    }