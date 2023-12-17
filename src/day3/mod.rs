struct Field {
    line : usize,   // line/row of the field
    column: usize,  // column of the field
}

struct PartNumber {
    num: u32,                     // part number
    line: usize,                  // line number the part number sits in
    adjacent_fields: Vec<Field>,  // all adjacent fields of the part number
}

impl PartNumber {
    fn has_adjacent_symbol(&self, lines: &Vec<&str>) -> bool {
        for field in &self.adjacent_fields {
            // get line of adjacent field
            let line: &[u8] = lines[field.line].as_bytes();

            // get character of adjacent field
            let chr: u8 = line[field.column];

            // check if the current adjacent field is a valid
            // symbol (not a number and not a dot '.');
            // if so, return true immediately
            if !chr.is_ascii_digit() && chr != b'.' {
                return true;
            }
        }

        // if none of the adjacent field was a valid symbol, return false
        return false;
    }
}

pub fn puzzle1(file_content: String) -> String {
    let mut engine_part_sum: u32 = 0;

    let file_lines: Vec<&str> =
        file_content.split("\n").map(|l| l.trim()).collect();

    for (i, line) in file_lines.iter().enumerate() {
        if line.is_empty() {  // skip empty lines
            continue;
        }

        // get all part numbers of the current line
        let part_numbers: Vec<PartNumber> = get_part_numbers(line, i);

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

// get all part numbers of the current line
fn get_part_numbers(line: &str, line_num: usize) -> Vec<PartNumber> {

    let mut part_numbers: Vec<PartNumber> = Vec::new();

    let mut start_of_part_num: usize = 0;
    let mut end_of_part_sum:   usize = 0;

    // treat line string as a byte slice/array, which is safe here
    // since input file only contains ASCII characters
    let line_array: &[u8] = line.as_bytes();
    let mut line_idx: usize = 0;

    // println!("Line: {line}");

    while line_idx < line.len() {
        if line_array[line_idx].is_ascii_digit() {
            // mark start index of the found part number
            start_of_part_num = line_idx;

            // keep going until the end of the current part number is reached
            while line_idx < line.len() &&
                  line_array[line_idx].is_ascii_digit() {
                line_idx += 1;
            }

            // mark end index of the found part number
            end_of_part_sum = line_idx;

            // convert part number string slice to actual number
            let part_number: u32 =
                (&line[start_of_part_num..end_of_part_sum]).parse().unwrap();

            // calculate all adjacent fields of the current part number
            let adjacent_fields: Vec<Field> =
                get_adjacent_fields(
                    line_num, start_of_part_num, end_of_part_sum);

            // add part number to vector
            part_numbers.push(PartNumber {
                num: part_number,
                line: line_num,
                adjacent_fields: adjacent_fields});
        }

        line_idx += 1;
    }

    return part_numbers;
}

// calculate all adjacent fields of a part number based
// on the index borders of the part number in the line it sits in
fn get_adjacent_fields(
    line: usize, left_idx: usize, right_idx: usize) ->Vec<Field> {
    let mut adjacent_fields: Vec<Field> = Vec::new();

    return adjacent_fields;
}
