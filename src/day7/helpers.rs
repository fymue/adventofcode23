use std::collections::HashMap;
use std::cmp::Ordering;

const VALID_CARDS: [char; 13] =
    ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Rank {
    FiveOfKind  = 7,
    FourofKind  = 6,
    FullHouse   = 5,
    ThreeOfKind = 4,
    TwoPair     = 3,
    SinglePair  = 2,
    HighCard    = 1,
}

impl Rank {
    // check if all cards are equal (since a consists of exaclty 5 cards)
    fn is_five_of_kind(hand: &str) -> bool {
        let mut hand = hand.bytes();
        let first_chr: u8 = hand.next().unwrap();
        return hand.all(|c| c == first_chr);
    }

    // if highest card count is 4, remaining card is irrelevant
    fn is_four_of_kind(hand: &str) -> bool {
        let (_counts, high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        return high_count == 4;
    }

    fn is_three_of_kind(hand: &str) -> bool {
        let (counts, _high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let mut pair_count: u8 = 0;
        let mut triplet_count: u8 = 0;

        for (_card, count) in counts {
            if count == 2 {
                pair_count += 1;
            } else if count == 3 {
                triplet_count += 1;
            }
        }

        return triplet_count == 1 && pair_count == 0;
    }

    fn is_full_house(hand: &str) -> bool {
        let (counts, _high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let mut pair_count: u8 = 0;
        let mut triplet_count: u8 = 0;

        for (_card, count) in counts {
            if count == 2 {
                pair_count += 1;
            } else if count == 3 {
                triplet_count += 1;
            }
        }

        return triplet_count == 1 && pair_count == 1;
    }

    fn is_two_pair(hand: &str) -> bool {
        let (counts, _high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let mut pair_count: u8 = 0;

        for (_card, count) in counts {
            if count == 2 {
                pair_count += 1;
            }
        }

        return pair_count == 2;
    }

    // if highest card count is 2, all other cards must be different
    fn is_single_pair(hand: &str) -> bool {
        let (counts, _high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let mut pair_count: u8 = 0;
        let mut triplet_count: u8 = 0;

        for (_card, count) in counts {
            if count == 2 {
                pair_count += 1;
            } else if count == 3 {
                triplet_count += 1;
            }
        }

        return pair_count == 1 && triplet_count == 0;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand<'a> {
    hand: &'a str,
    bid: u32,
    rank: Rank,
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
    pub fn beats(&self, other: &Hand) -> bool {
        if self.rank == other.rank {
            return self.is_hand_stronger(other);
        } else {
            return self.rank > other.rank;
        }
    }

    // check if hand is stronger than the other hand if they
    // both share the same rank (i.e. both hands contain 2 pairs)
    fn is_hand_stronger(&self, other: &Hand) -> bool {
        let rank_map: HashMap<char, u8> = get_card_ranks();

        for (self_card, other_card) in
            self.hand.chars().zip(other.hand.chars()) {
            if self_card != other_card {
                return rank_map[&self_card] > rank_map[&other_card];
            }
        }

        assert!(self.hand != other.hand);
        return false;
    }
}

pub fn parse_hand<'a> (line: &'a str) -> Hand<'a> {
    const DELIMITER: &str = " ";

    assert!(line.find(DELIMITER).is_some());
    let delimiter_idx: usize = line.find(DELIMITER).unwrap();

    let hand: &str = &line[0..delimiter_idx];
    assert!(is_valid_hand(hand));

    let bid: u32 = (&line[delimiter_idx+1..]).parse().unwrap();
    let rank: Rank = get_rank(hand);

    return Hand{hand: hand, bid: bid, rank: rank};
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

fn get_rank(hand: &str) -> Rank {
    let rank: Rank = if Rank::is_five_of_kind(hand) {
        Rank::FiveOfKind
    } else if Rank::is_four_of_kind(hand) {
        Rank::FourofKind
    } else if Rank::is_full_house(hand) {
        Rank::FullHouse
    } else if Rank::is_three_of_kind(hand) {
        Rank::ThreeOfKind
    } else if Rank::is_two_pair(hand) {
        Rank::TwoPair
    } else if Rank::is_single_pair(hand) {
        Rank::SinglePair
    } else {
        Rank::HighCard
    };

    return rank;
}

fn count_cards(hand: &str) -> (HashMap<char, u8>, u8) {
    let mut card_counts: HashMap<char, u8> = HashMap::new();

    let mut highest_card_count: u8 = 0;

    for card in hand.chars() {
        let entry: Option<&mut u8> = card_counts.get_mut(&card);

        if entry.is_some() {
            let count: &mut u8 = entry.unwrap();
            *count += 1;

            if *count > highest_card_count {
                highest_card_count = *count;
            }
        } else {
            card_counts.insert(card, 1);
        }
    }

    return (card_counts, highest_card_count);
}

fn is_valid_hand(hand: &str) -> bool {
    return hand.chars().all(|c| VALID_CARDS.contains(&c));
}

#[inline(always)]
fn get_card_ranks() -> HashMap<char, u8> {
    let rank_map: HashMap<char, u8> = HashMap::from([
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

    return rank_map;
}
