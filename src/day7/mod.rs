mod helpers;

use std::collections::HashMap;
use helpers::Hand;

pub fn puzzle1(file_content: String) -> String {
    let with_joker: bool = false;
    let card_ranks: HashMap<char, u8> = helpers::get_card_ranks_puzzle1();
    let total_winnings: u32 =
        solve_puzzle(file_content, with_joker, &card_ranks);

    return total_winnings.to_string();
}

pub fn puzzle2(file_content: String) -> String {
    let with_joker: bool = true;
    let card_ranks: HashMap<char, u8> = helpers::get_card_ranks_puzzle2();
    let total_winnings: u32 =
        solve_puzzle(file_content, with_joker, &card_ranks);

    return total_winnings.to_string();
}

fn solve_puzzle(
    file_content: String, with_joker: bool,
    card_ranks: &HashMap<char, u8>) -> u32 {
    let mut hands: Vec<Hand> = Vec::new();

    // parse all the hands and bids; determine rank of each hand
    for mut line in file_content.split("\n") {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        hands.push(helpers::parse_hand(line, with_joker));
    }

    // sort the hands by their rank in increasing order
    // (i.e. best hand is rightmost element of vector)
    hands.sort_by(|a, b| a.compare(b, card_ranks));

    // calculate the total winnings by multplying
    // the rank of each hand with its bidding value
    let total_winnings: u32 = helpers::calc_total_winnings(&hands);

    return total_winnings;
}
