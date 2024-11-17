mod solutions;
mod utils;

fn main() {
    let day = std::env::args().nth(1).expect("no day provided");
    let input = utils::read_input(day.as_ref());

    match day.as_ref() {
        "01" => solutions::solution_01::solve(input),
        _    => println!("That day is not yet solved. :-("),
    }
}
