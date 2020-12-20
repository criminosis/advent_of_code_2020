use std::{collections::HashSet, fs};

pub fn part_one() {
    let input =
        fs::read_to_string("inputs/5_part1.txt").expect("Something went wrong reading the file");
    let max_id = input.lines().map(|line| process_row(line)).max().unwrap();
    println!("Max Seat Id: {}", max_id);
}

pub fn part_two() {
    let input =
        fs::read_to_string("inputs/5_part1.txt").expect("Something went wrong reading the file");
    // minimum id -> 0
    // maximum id -> 127 * 8 + 7 = 1023
    let mut possible_ids = HashSet::new();
    for possible_id in 0..1023 {
        possible_ids.insert(possible_id);
    }

    for an_actual_id in input.lines().map(|line| process_row(line)).into_iter() {
        possible_ids.remove(&an_actual_id);
    }

    let mut remaining_ids:Vec<i32> = possible_ids.into_iter().collect();
    remaining_ids.sort();

    //My seat will be the one not contiguous of the "non-existent" seat ids in the front / back of the plane
    println!("Open seats: {:?}", remaining_ids);
}


pub fn middle_of (left: i32, right:i32) -> i32 {
    return ((right - left) as f64/ 2.0).ceil() as i32;
}


pub fn process_row(input: &str) -> i32 {
    let mut top_row = 0;
    let mut bottom_row = 127;
    let mut left_column = 0;
    let mut right_column = 7;

    for seat_selector in input.chars().into_iter() {
        match seat_selector {
            'F' => bottom_row = bottom_row - middle_of(top_row, bottom_row),
            'B' => top_row = top_row + middle_of(top_row, bottom_row),
            'L' => right_column = right_column - middle_of(left_column, right_column),
            'R' => left_column = left_column + middle_of(left_column, right_column),
            _ => panic!("UNKNOWN CHAR")
        }

        //println!("top_row: {} & bottom_row {}", top_row, bottom_row);
        //println!("left_column: {} & right_column {}", left_column, right_column);
    }

    // multiply the row by 8, then add the column
    //println!("top_row: {} & bottom_row {}", top_row, bottom_row);
    //println!("left_column: {} & right_column {}", left_column, right_column);
    let seat_id = top_row * 8 + left_column;
    //println!("Seat Id: {}", seat_id);
    return seat_id;
}
