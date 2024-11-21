mod solutions;
mod utils;
use std::collections::HashMap;

fn _daily_results(day: String) -> (i32, i32) {
    let input = utils::read_input(day.as_ref());
    match day.as_ref() {
        "01" => return solutions::solution_01::solve(input),
        "02" => return solutions::solution_02::solve(utils::get_input_lines(input.as_ref())),
        &_ => return (0, 0),
    }
}

fn test() {
    let answers = HashMap::from([
        ("01", (i32::from(138), i32::from(1771))),
        ("02", (i32::from(1586300), i32::from(0)))
        ]);
    for (day, expected_results) in &answers {
        let actual_results = _daily_results(day.to_string());
        assert_eq!(expected_results, &actual_results, "Testing Day: {day}");
        println!("Testing Day: {day} - {:?}", expected_results);
    }
}

fn main() {
    let day = std::env::args().nth(1).expect("no day provided");

    match day.as_ref() {
        "test" => test(),
        "01" => {
            let results = _daily_results(day);
            println!("Floor: {:?}", results.0);
            println!("Basement: {:?}", results.1);
        }
        "02" => {
            let results = _daily_results(day);
            println!("Wrapping Paper Needed: {:?}", results.0);
            println!("Ribbon Needed: {:?}", results.1);
        }
        &_ => println!("That day is not yet solved. :-("),
    }
}
