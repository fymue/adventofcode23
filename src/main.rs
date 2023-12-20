use std::env;
use std::process::exit;
use std::{fs::File, io::Read};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

    let mut input_file: File =
        File::open(input_file_path).expect(
            &format!("Couldn't open input file '{}'!", input_file_path));

    let mut file_content: String = String::new();
    input_file.read_to_string(&mut file_content).expect(
        &format!("Couldn't read input file '{}'!", input_file_path));

    let result: String = match aoc_day {
        1 => match aoc_puzzle_of_day {
            1 => day1::puzzle1(file_content),
            2 => day1::puzzle2(file_content),
            _ => invalid_puzzle_num_str
        },

        2 => match aoc_puzzle_of_day {
            1 => day2::puzzle1(file_content),
            2 => day2::puzzle2(file_content),
            _ => invalid_puzzle_num_str
        },

        3 => match aoc_puzzle_of_day {
            1 => day3::puzzle1(file_content),
            2 => day3::puzzle2(file_content),
            _ => invalid_puzzle_num_str,
        },

        4 => match aoc_puzzle_of_day {
            1 => day4::puzzle1(file_content),
            2 => day4::puzzle2(file_content),
            _ => invalid_puzzle_num_str,
        },

        5 => match aoc_puzzle_of_day {
            1 => day5::puzzle1(file_content),
            _ => invalid_puzzle_num_str,
        },

        _ => String::from("Invalid AOC day number")
    };

    println!("Result of AOC day {}, puzzle {}: {}",
             aoc_day, aoc_puzzle_of_day, result);
}
