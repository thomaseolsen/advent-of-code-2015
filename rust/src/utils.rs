use std::fs;

pub fn read_input(day: &str) -> String {
    let mut path = String::from("../input/");
    path.push_str(day);
    path.push_str("_input.txt");
    let input = fs::read_to_string(path).expect("path should provide input");
    return input;
}