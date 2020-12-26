use crate::utils;
use std::collections::HashMap;

pub fn part_one() {
    let group_data = utils::group_data_by_empty_lines(&utils::read_file("inputs/6_part1.txt"));
    let result: usize = group_data.iter()
    .map(|x| x.chars().collect())
    .map(|x: Vec<char>| {
        let mut owned = x.to_owned();
        owned.sort();
        owned.dedup();
        return owned;
    })
    .map(|x| x.len())
    .sum();
    println!("{}", result);
}

pub fn part_two() {
    let input = utils::read_file("inputs/6_part1.txt");
    let mut grouped_input: Vec<(String, i32)> = vec![];
    let mut accumulation = String::new();
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            grouped_input.push((accumulation.clone(), count));
            accumulation.clear();
            count = 0;
        } else {
            count = count + 1;
            accumulation.push_str(line);
        }
    }

    if !accumulation.is_empty() {
        grouped_input.push((accumulation.clone(), count));
    }

    let result: usize = grouped_input.iter().map(|(line, responses)| {
        let mut m = HashMap::new();
        for answer_char in line.chars() {
            *m.entry(answer_char).or_insert(0) +=1;
        }

        return m.iter()
        //how many answerChars occured equal to the number of responses within that group
        .filter(|(_, count)| *count == responses)
        .count();
    })
    .sum();

    println!("Answer {}", result);
}