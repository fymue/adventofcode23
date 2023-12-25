mod helpers;
use helpers::Hand;

pub fn puzzle1(file_content: String) -> String {
    let mut hands: Vec<Hand> = Vec::new();

    // parse all the hands and bids; determine rank of each hand
    for mut line in file_content.split("\n") {
        line = line.trim();
        if line.is_empty() {
            continue;
        }

        hands.push(helpers::parse_hand(line));
    }

    // sort the hands by their rank in increasing order
    // (i.e. best hand is rightmost element of vector)
    hands.sort_by(|a, b| a.cmp(b));

    // calculate the total winnings by multplying
    // the rank of each hand with its bidding value
    let total_winnings: u32 = helpers::calc_total_winnings(&hands);

    return total_winnings.to_string();
}
