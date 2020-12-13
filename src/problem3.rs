use std::fs;

pub fn part_one() {
    let unit_rows: Vec<Vec<char>> = fs::read_to_string("inputs/3_part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    //we need to repeat unit_columns into our map
    let mut map: Vec<Vec<char>> = unit_rows.iter().map(|line| line.clone()).collect();

    let mut location = (0, 0);
    let movement = (3, 1);
    let mut trees_encountered = 0;

    //keep going until we reach the bottom
    while location.1 < unit_rows.len() {

        //if we're about to fall off the right edge, then add more
        if location.0 + movement.0 > map[0].len() {
            println!("Expanding at {}. Current length {}. Adding {}", location.0, map[0].len(), unit_rows[0].len());
            for (index, unit_row) in unit_rows.iter().enumerate() {
                map[index].append(&mut (unit_row.clone()));
            }
        }

        //see if we're at a tree
        println!("Checking at {}, {}", location.0, location.1);
        if map[location.1][location.0] == '#' {
            trees_encountered = trees_encountered + 1;
            println!("Encountered tree at {}, {}", location.0, location.1);
        }

        //now move to our new spot
        location.0 = location.0 + movement.0;
        location.1 = location.1 + movement.1;
    }

    println!("Total trees: {}", trees_encountered)
}