use std::fs;

pub fn group_data_by_empty_lines(input: &String) -> Vec<String> {
    let mut grouped_input: Vec<String> = vec![];
    let mut accumulation = String::new();
    for line in input.lines() {
        if line.is_empty() {
            grouped_input.push(accumulation.clone());
            accumulation.clear();
        } else {
            accumulation.push_str(line);
        }
    }

    if !accumulation.is_empty() {
        grouped_input.push(accumulation.clone());
    }

    return grouped_input;
}


pub fn read_file(filepath: &str) -> String {
        return fs::read_to_string(filepath).expect("Something went wrong reading the file");
}
