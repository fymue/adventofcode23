// Solutions for https://adventofcode.com/2023/day/1

mod helpers;

use crate::day1::helpers::get_first_number_char;
use crate::day1::helpers::get_first_number_word;
use crate::day1::helpers::get_last_number_word;

pub fn puzzle1(file_content: String) -> String {
    let mut total_sum: u32 = 0;

    for mut line in file_content.split("\n") {
        line = line.trim();   // strip line of whitespace
        if line.is_empty() {  // skip empty lines
            continue;
        }

        let first_num: u32 = get_first_number_char(line.chars()).unwrap();
        let last_num:  u32 = get_first_number_char(line.chars().rev()).unwrap();

        // combine first and last number and add them to total sum
        total_sum += first_num * 10 + last_num;
    }

    return total_sum.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    let mut total_sum: u32 = 0;

    for mut line in file_content.split("\n") {
        line = line.trim();   // strip line of whitespace
        if line.is_empty() {  // skip empty lines
            continue;
        }

        let first_num: u32 = get_first_number_word(line).unwrap();
        let last_num:  u32 = get_last_number_word(line).unwrap();

        // combine first and last number and add them to total sum
        total_sum += first_num * 10 + last_num;
    }

    return total_sum.to_string();
}
