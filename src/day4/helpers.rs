use std::collections::HashSet;

pub const NUMBER_DELIMITER: &str = " ";
pub const LINE_DELIMITER: &str = " | ";

const CARDID_START: usize = 1;

// strip CardID ("Card x: ") from line and return the CardID number "x"
pub fn strip_cardid(line: &str) -> (u32, &str) {
    const CARDID_END_STR: &str = ": ";
    const CARDID_STR: &str = "Card ";

    assert!(line.find(CARDID_STR).is_some());
    assert!(line.find(CARDID_END_STR).is_some());

    let cardid_start_idx: usize =
        line.find(CARDID_STR).unwrap() + CARDID_STR.len();

    let cardid_end_idx: usize =
        line.find(CARDID_END_STR).unwrap();

    let line_without_cardid: &str =
        &line[cardid_end_idx + CARDID_END_STR.len()..];
    
    let cardid_num: u32 =
        ((&line[cardid_start_idx..cardid_end_idx]).trim()).parse().unwrap();

    return (cardid_num, line_without_cardid);
}

// parse the winning numbers from the current line and
// add them to a set for fast lookups later
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

// isolate the list of drawn numbers from the line
pub fn get_drawn_numbers_str(line: &str) -> &str {
    assert!(line.find(LINE_DELIMITER).is_some());

    let line_delim_end_idx: usize =
        line.find(LINE_DELIMITER).unwrap() + LINE_DELIMITER.len();

    return &line[line_delim_end_idx..];
}

pub fn collect_won_scratchcards(
    drawn_numbers: &str, card: u32, winning_numbers: &HashSet<u32>,
    card_copies: &mut Vec<HashSet<u32>>) {
    // counter for all won scratchcards of current card
    let mut won_scratchcard: u32 = card;

    // add the current card to the hashmap with an initial count of 1
        // since we've just found the first (original) copy of the card
    card_copies.push(HashSet::new());

    // iterate over all drawn numbers
    for mut num_str in drawn_numbers.split(NUMBER_DELIMITER) {
        num_str = num_str.trim();
        if num_str.is_empty() {
            continue;
        }

        // parse the current drawn number
        let num: u32 = num_str.parse().unwrap();
        if winning_numbers.contains(&num) {
            won_scratchcard += 1;

            // get mutable reference to the set of the current scratchcard
            // that stores all additional won copies of other scratchcards
            let won_cards: &mut HashSet<u32> = &mut card_copies[card as usize];

            // add the won additional scratchcard to the set
            won_cards.insert(won_scratchcard);
        }
    }
}

// count the total scratchcards, including all the won scratchcards
pub fn count_total_scratchcards(card_copies: &Vec<HashSet<u32>>) -> u32 {
    // stores counts for all scratchcards based in their IDs (starting from 1)
    let mut card_counts = vec![1; card_copies.len()];

     // first value in vector is just there to make indexing directly
     // with CardIDs possible (CardIDs start from 1), so this value
     // needs to be ignored in the final summing of all counts, which
     // is why it it set to 0 here
    card_counts[0] = 0;

    // iterate over all cards and count all the won scratchcards
    for card in CARDID_START..card_copies.len() {

        for cmp_card in CARDID_START..card_copies.len() {
            if card == cmp_card {
                continue;
            }

            let won_cards: &HashSet<u32> = &card_copies[cmp_card];
            let cmp_card_count: u32 = card_counts[cmp_card];

            // if the current card appears in the won cards for the current
            // comparison card, increment its counter by the counter of
            // the comparison card
            // (assumes that processing of file is top to bottom)
            if won_cards.contains(&(card as u32)) {
                let card_count: &mut u32 = &mut card_counts[card];
                *card_count += cmp_card_count;
            }
        }
    }

    // sum up all the final card counts for every card
    let total_scratchcards: u32 = card_counts.iter().sum();

    return total_scratchcards;
}
