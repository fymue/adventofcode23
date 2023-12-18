// Solutions for https://adventofcode.com/2023/day/3

use std::collections::HashMap;

mod helpers;

use crate::day3::helpers::PartNumber;
use crate::day3::helpers::Field;
use crate::day3::helpers::get_part_numbers;
use crate::day3::helpers::get_symbol_from_line;
use crate::day3::helpers::calc_gear_ratio_sum;

pub fn puzzle1(file_content: String) -> String {
    let mut engine_part_sum: u32 = 0;

    // collect all lines of file into vector, trimming off whitespace
    // and skipping empty lines
    let file_lines: Vec<&str> =
        file_content.split("\n").map(|l| l.trim()).
            filter(|l| !l.is_empty()).collect();

    for (i, line) in file_lines.iter().enumerate() {
        if line.is_empty() {  // skip empty lines
            continue;
        }

        // get all part numbers of the current line
        let part_numbers: Vec<PartNumber> = get_part_numbers(&file_lines, i);

        // iterate over all part numbers of current line and check if it
        // has an adjacent symbol; if so, add part number to total sum
        for part_number in part_numbers {
            if part_number.has_adjacent_symbol(&file_lines) {
                engine_part_sum += part_number.num;
            }
        }        
    }

    return engine_part_sum.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    const GEAR_SYMBOL: u8 = b'*';
    let mut potential_gears: HashMap<Field, Vec<u32>> = HashMap::new();

    // collect all lines of file into vector, trimming off whitespace
    // and skipping empty lines
    let file_lines: Vec<&str> =
        file_content.split("\n").map(|l| l.trim()).
            filter(|l| !l.is_empty()).collect();

    for (i, line) in file_lines.iter().enumerate() {
        if line.is_empty() {  // skip empty lines
            continue;
        }

        // get all part numbers of the current line
        let part_numbers: Vec<PartNumber> = get_part_numbers(&file_lines, i);

        // iterate over all part numbers of current line and check if it
        // has an adjacent symbol; if so, add part number to total sum
        for part_number in part_numbers {
            for field in part_number.adjacent_fields {
                // get symbol of current field
                let curr_symbol: u8 = get_symbol_from_line(&field, &file_lines);

                // if current symbol is a gear ('*'), add it to the gear map
                // and add the current part number to its adjacent number list
                if curr_symbol == GEAR_SYMBOL {
                    match potential_gears.get_mut(&field) {
                        Some(num_map) => num_map.push(part_number.num),

                        // if the gear isn't part of the map yet,
                        // add it to the map and add the current
                        // part number to its adjacent number list
                        None =>
                            match potential_gears.insert(
                                field, vec![part_number.num]) {
                                    Some(_) => (),
                                    None => (),
                            }
                    }
                }
            }
        }
    }

    let gear_ration_sum: u32 = calc_gear_ratio_sum(potential_gears);

    return gear_ration_sum.to_string();
}
