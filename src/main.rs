mod days;
mod utils;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args.get(1).expect("usage: cargo run -- <day>").parse().expect("day must be a number");

    match day {
        1 => days::day1::run(),
        _ => println!("Day not implemented yet"),
    }
}


