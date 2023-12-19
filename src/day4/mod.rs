// Solutions to https://adventofcode.com/2023/day/4

use std::collections::HashSet;

mod helpers;

use crate::day4::helpers::NUMBER_DELIMITER;

use crate::day4::helpers::strip_cardid;
use crate::day4::helpers::get_winning_numbers;
use crate::day4::helpers::get_drawn_numbers_str;
use crate::day4::helpers::collect_won_scratchcards;
use crate::day4::helpers::count_total_scratchcards;

pub fn puzzle1(file_content: String) -> String {
    let mut total_points: u32 = 0;

    for mut line in file_content.split("\n") {
        line = line.trim();   // strip whitespace
        if line.is_empty() {  // skip empty lines
            continue;
        }

        let mut winning_number_count: u32 = 0;

        // get the line content without the CardID part
        let (_cardid_num, line_without_id): (u32, &str) = strip_cardid(line);

        // collect all winning numbers in a set
        let winning_numbers: HashSet<u32> =
            get_winning_numbers(line_without_id);

        // isolate just the drawn numbers from the line
        let drawn_numbers_str: &str = get_drawn_numbers_str(line_without_id);

        // iterate over all drawn numbers and do a simple lookup if
        // the current drawn number is part of the winning numbers;
        // if so, update the count
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

        // calculate the points for the current card
        let points: u32 = if winning_number_count == 0 {
            0
        } else {
            u32::pow(2, winning_number_count - 1)
        };

        total_points += points; 
    }

    return total_points.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    // stores all won copies for every card;
    // each vector index corresponds to a CardID;
    // since the input file provides incremental CardIDs starting from 1,
    // we can use the vector indices to look up a CardID
    let mut card_copies: Vec<HashSet<u32>> = Vec::new();

    // push empty value into vector since CardIDs start from 1 and not from 0
    card_copies.push(HashSet::new());

    for mut line in file_content.split("\n") {
        line = line.trim();   // strip whitespace
        if line.is_empty() {  // skip empty lines
            continue;
        }

        let (cardid_num, line_without_id): (u32, &str) = strip_cardid(line);

        let winning_numbers: HashSet<u32> =
            get_winning_numbers(line_without_id);

        let drawn_numbers_str: &str = get_drawn_numbers_str(line_without_id);

        // collect all won scratchcards for the current card
        collect_won_scratchcards(
            drawn_numbers_str, cardid_num, &winning_numbers, &mut card_copies);
    }

    // count the total scratchcards, including all the won scratchcards
    let total_scratchcards: u32 = count_total_scratchcards(&card_copies);

    return total_scratchcards.to_string();
}
