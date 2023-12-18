use std::collections::HashSet;

pub const NUMBER_DELIMITER: &str = " ";
pub const LINE_DELIMITER: &str = " | ";

// strip CardID from line
pub fn strip_cardid(line: &str) -> &str {
    const CARDID_END_STR: &str = ": ";

    assert!(line.find(CARDID_END_STR).is_some());
    let cardid_end_idx: usize =
        line.find(CARDID_END_STR).unwrap() + CARDID_END_STR.len();

    return &line[cardid_end_idx..];
}

pub fn get_winning_numbers(line: &str) -> HashSet<u32> {
    assert!(line.find(LINE_DELIMITER).is_some());

    let mut winning_numbers: HashSet<u32> = HashSet::new();

    let line_delim_idx: usize = line.find(LINE_DELIMITER).unwrap();

    let winning_number_str: &str = &line[0..line_delim_idx];

    for mut num_str in winning_number_str.split(NUMBER_DELIMITER) {
        num_str = num_str.trim();
        if num_str.is_empty() {
            continue;
        }

        assert!(num_str.parse::<u32>().is_ok());

        let num: u32 = num_str.parse().unwrap();
        winning_numbers.insert(num);
    }

    return winning_numbers;
}

pub fn get_drawn_numbers_str(line: &str) -> &str {
    assert!(line.find(LINE_DELIMITER).is_some());

    let line_delim_end_idx: usize =
        line.find(LINE_DELIMITER).unwrap() + LINE_DELIMITER.len();

    return &line[line_delim_end_idx..];
}