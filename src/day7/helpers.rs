use std::collections::HashMap;
use std::cmp::Ordering;
use std::ops::BitAnd;

#[derive(PartialEq, PartialOrd)]
enum Rank {
    FiveOfKind  = 7,
    FourofKind  = 6,
    FullHouse   = 5,
    ThreeOfKind = 4,
    TwoPair     = 3,
    SinglePair  = 2,
    HighCard    = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand<'a> {
    hand: &'a str,
    bid: u32,

}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand == other.hand {
            return Ordering::Equal;
        }

        if self.beats(other) {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

impl Hand<'_> {
    pub fn beats(&self, other_hand: &Hand) -> bool {
        let own_rank: Rank = self.get_rank();
        let other_rank: Rank = other_hand.get_rank();

        if own_rank == other_rank {
            return self.is_hand_stronger(other_hand);
        } else {
            return own_rank > other_rank;
        }
    }

    fn get_rank(&self) -> Rank {
        // TODO: parse hand string and return the rank of the hand
    }

    // check if hand is stronger than the other hand if they
    // both share the same rank (i.e. both hands contain 2 pairs)
    fn is_hand_stronger(&self, other_hand: &Hand) -> bool {
        // TODO: compare each card for both hands and return if
        //       the self card has the stronger hand
    }
}

pub fn parse_hand(line: &str) -> Hand {
    const DELIMITER: &str = " ";

    assert!(line.find(DELIMITER).is_some());
    let delimiter_idx: usize = line.find(DELIMITER).unwrap();

    let hand: &str = &line[0..delimiter_idx];
    let bid: u32 = (&line[delimiter_idx+1..]).parse().unwrap();

    return Hand{hand: hand, bid: bid};
}

pub fn calc_total_winnings(ranked_hands: &Vec<Hand>) -> u32 {
    let mut total_winnings: u32 = 0;

    for (i, hand) in ranked_hands.iter().enumerate() {
        // since hands Vector is sorted, the indices of the hands (+1)
        // represent their final rank
        let final_rank_of_hand: u32 = (i + 1) as u32;
        total_winnings += final_rank_of_hand * hand.bid;
    }

    return total_winnings;
}
