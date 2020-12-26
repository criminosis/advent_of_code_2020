use crate::utils;

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