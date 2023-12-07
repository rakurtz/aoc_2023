// Advent of Code 2023 in Rust
use std::env;

const USAGE: &str = "\nNo argument given, what day's exercise to run.
Usage: e.g. `cargo run --1` for day 1\n";

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(day) = args.get(1) {
        match day.parse::<usize>() {
            Ok(1) => aoc_2023::solutions::day1::run(),
            Ok(2) => aoc_2023::solutions::day2::run(),
            Ok(3) => aoc_2023::solutions::day3::run(),
            Ok(4) => aoc_2023::solutions::day4::run(),
            Ok(5) => aoc_2023::solutions::day5::run(),
            Ok(6) => aoc_2023::solutions::day6::run(),

            _ => println!("{}", USAGE),
        }
    } else {
        println!("{}", USAGE);
    }
}
