use std::fs;

pub fn read_input(day: &str) -> String {
    let mut path = String::from("../input/");
    path.push_str(day);
    let input = fs::read_to_string(path).expect("path should provide input");
    return input;
}

pub fn get_input_lines(input: &str) -> Vec<&str> {
    return input.split("\n").collect();
}