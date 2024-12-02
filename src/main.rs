mod utils;

mod day01;
mod day02;
mod day03;

fn main() {
    let day = std::env::args().nth(1).expect("Please provide day as a CLI argument");

    match day.trim() {
        "day01" => day01::main(),
        "day02" => day02::main(),
        "day03" => day03::main(),

        _ => {
            println!("Undefined day");
        }
    }
}