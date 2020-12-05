use std::fs;

pub fn part_one() {
    let mut contents: Vec<u32> = fs::read_to_string("inputs/1_part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
        let (x, y) = find_2020_pair(&mut contents);
        println!("Found: {} & {}. Makes {}", x, y, x * y);
}


fn find_2020_pair(data: &mut Vec<u32>) -> (u32, u32) {
    //keep removing the first value until we're empty and see if it plus anything left in the Vec adds up to 2020
    while !data.is_empty() {
        let test_value = data.remove(0);
        for other_value in data.iter() {
            if test_value + other_value == 2020 {
                return (test_value, other_value.clone());
            }
        }
    }
    panic!("Failed to find value!")
}