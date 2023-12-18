// Solutions to https://adventofcode.com/2023/day/4

use std::collections::HashSet;

mod helpers;

use crate::day4::helpers::NUMBER_DELIMITER;
use crate::day4::helpers::strip_cardid;
use crate::day4::helpers::get_winning_numbers;
use crate::day4::helpers::get_drawn_numbers_str;

pub fn puzzle1(file_content: String) -> String {
    let mut total_points: u32 = 0;

    for mut line in file_content.split("\n") {
        line = line.trim();   // strip whitespace
        if line.is_empty() {  // skip empty lines
            continue;
        }

        let mut winning_number_count: u32 = 0;

        let line_without_id: &str = strip_cardid(line);

        let winning_numbers: HashSet<u32> =
            get_winning_numbers(line_without_id);

        let drawn_numbers_str: &str = get_drawn_numbers_str(line_without_id);

        for mut num_str in drawn_numbers_str.split(NUMBER_DELIMITER) {
            num_str = num_str.trim();
            if num_str.is_empty() {
                continue;
            }

            let num: u32 = num_str.parse().unwrap();
            if winning_numbers.contains(&num) {
                winning_number_count += 1;

            }
        }

        let points: u32 = if winning_number_count == 0 {
            0
        } else {
            u32::pow(2, winning_number_count - 1)
        };

        total_points += points; 
    }

    return total_points.to_string();
}
