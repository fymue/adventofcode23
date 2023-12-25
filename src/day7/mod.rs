mod helpers;

use std::collections::HashMap;
use helpers::Hand;

pub fn puzzle1(file_content: String) -> String {
    let rank_map: HashMap<char, u32> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    let mut hands: Vec<Hand> = Vec::new();

    for mut line in file_content.split("\n") {
        line = line.trim();
        hands.push(helpers::parse_hand(line, &rank_map));
    }

    hands.sort();
    println!("{:?}", hands);

    let total_winnings: u32 = helpers::calc_total_winnings(&hands);

    return total_winnings.to_string();
}
