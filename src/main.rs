mod utils;

mod day01;
mod day02;

fn main() {
    let day = std::env::args().nth(1).expect("Please provide day as a CLI argument");

    match day.trim() {
        "day01" => day01::main(),
        "day02" => day02::main(),

        _ => {
            println!("Undefined day");
        }
    }
}