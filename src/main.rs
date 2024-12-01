mod day01;
mod utils;

fn main() {
    let day = std::env::args().nth(1).expect("Please provide day as a CLI argument");

    match day.trim() {
        "day01" => day01::main(),

        _ => {
            println!("Undefined day");
        }
    }
}