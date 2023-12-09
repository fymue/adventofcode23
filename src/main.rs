use std::env;
use std::process::exit;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 ||
       args.len() == 2 && ["-h", "--help"].contains(&args[1].as_str()) {
        println!("Usage: cargo run DAY PUZZLE path/to/input.txt");
        exit(0);
    }

    // get Advent of Code day
    let aoc_day: u8 = args[1].parse().expect(
        &format!("Provided AOC day need to be between 1 and 25!"));

    let aoc_puzzle_of_day: u8 = args[2].parse().unwrap();  // puzzle number

    let input_file_path: &str = args[3].as_str();  // path to input file

    let invalid_puzzle_num_str: String = String::from("Invalid puzzle number");

    let result: String = match aoc_day {
        1 => match aoc_puzzle_of_day {
            1 => day1::day1_puzzle1(input_file_path),
            _ => invalid_puzzle_num_str
        },
        _ => String::from("Invalid AOC day number")
    };

    println!("Result of AOC day {}, puzzle {}: {}",
             aoc_day, aoc_puzzle_of_day, result);
}
