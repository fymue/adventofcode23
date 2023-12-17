use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Field {
    line : usize,   // line/row of the field
    column: usize,  // column of the field
}

struct PartNumber {
    num: u32,                     // part number
    adjacent_fields: Vec<Field>,  // all adjacent fields of the part number
}

impl PartNumber {
    fn has_adjacent_symbol(&self, lines: &Vec<&str>) -> bool {
        for field in &self.adjacent_fields {
            let symbol: u8 = get_symbol_from_line(field, lines);

            // check if the current adjacent field is a valid
            // symbol (not a number and not a dot '.');
            // if so, return true immediately
            if !symbol.is_ascii_digit() && symbol != b'.' {
                return true;
            }
        }

        // if none of the adjacent field was a valid symbol, return false
        return false;
    }
}

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

// get all part numbers of the current line
fn get_part_numbers(lines: &Vec<&str>, line_num: usize) -> Vec<PartNumber> {
    // maximum line column (assumes that all lines of file are of same length)
    let max_right_idx: usize = lines[0].len() - 1;

    // maximum line idx
    let max_line: usize = lines.len() - 1;

    let mut part_numbers: Vec<PartNumber> = Vec::new();

    let mut i_part_num: usize;
    let mut j_part_num: usize;

    // treat line string as a byte slice/array, which is safe here
    // since input file only contains ASCII characters
    let line_array: &[u8] = lines[line_num].as_bytes();
    let mut line_idx: usize = 0;

    while line_idx <= max_right_idx {
        if line_array[line_idx].is_ascii_digit() {
            // mark start index of the found part number
            i_part_num = line_idx;

            // keep going until the end of the current part number is reached
            while line_idx <= max_right_idx &&
                  line_array[line_idx].is_ascii_digit() {
                line_idx += 1;
            }

            // mark end index of the found part number
            j_part_num = line_idx - 1;

            // convert part number string slice to actual number
            let part_number: u32 =
                lines[line_num][i_part_num..=j_part_num].parse().unwrap();

            // calculate all adjacent fields of the current part number
            let adjacent_fields: Vec<Field> =
                get_adjacent_fields(
                    line_num, i_part_num, j_part_num, max_right_idx, max_line);

            // add part number to vector
            part_numbers.push(
                PartNumber{num: part_number, adjacent_fields: adjacent_fields});
        }

        line_idx += 1;
    }

    return part_numbers;
}

// calculate all adjacent fields of a part number based
// on the index borders of the part number in the line it sits in
fn get_adjacent_fields(
    line: usize, left_idx: usize, right_idx: usize,
    max_right_idx: usize, max_line: usize) ->Vec<Field> {
    let mut adjacent_fields: Vec<Field> = Vec::new();

    let left_border: usize = left_idx - (left_idx > 0) as usize;
    let right_border: usize = right_idx + (right_idx < max_right_idx) as usize;

    let lower_line: usize = line - (line > 0) as usize;
    let upper_line: usize = line + (line < max_line) as usize;

    for l in lower_line..=upper_line {
        for c in left_border..=right_border {
            adjacent_fields.push(Field{line: l, column: c});
        }
    }

    return adjacent_fields;
}

// return the symbol/character of a Field object
fn get_symbol_from_line(field: &Field, lines: &Vec<&str>) -> u8 {
    // get line of adjacent field
    let line: &[u8] = lines[field.line].as_bytes();

    // get character of adjacent field
    let symbol: u8 = line[field.column];

    return symbol;
}

// calculate the gear ration sum
fn calc_gear_ratio_sum(potential_gears: HashMap<Field, Vec<u32>>) -> u32 {
    const GEAR_PARTS: usize = 2;

    let mut gear_ration_sum: u32 = 0;

    for (_gear, part_numbers) in potential_gears {
        let is_gear: bool = part_numbers.len() == GEAR_PARTS;

        if is_gear {
            gear_ration_sum += part_numbers[0] * part_numbers[1];
        }
    }

    return gear_ration_sum;
}
