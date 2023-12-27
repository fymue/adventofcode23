use std::collections::HashMap;
use std::cmp::Ordering;

const VALID_CARDS: [char; 13] =
    ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
enum Rank {
    FiveOfKind  = 7,
    FourOfKind  = 6,
    FullHouse   = 5,
    ThreeOfKind = 4,
    TwoPair     = 3,
    SinglePair  = 2,
    HighCard    = 1,
}

impl Rank {
    // check if all cards are equal (since a consists of exaclty 5 cards)
    fn is_five_of_kind(hand: &str) -> bool {
        let (_counts, high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let is_five_of_kind: bool = high_count == 5;

        return is_five_of_kind;
    }

    fn is_four_of_kind(hand: &str) -> bool {
        let (_counts, high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let is_four_of_kind: bool = high_count == 4;
        
        return is_four_of_kind;
    }

    fn is_three_of_kind(hand: &str) -> bool {
        let (_counts, high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let is_three_of_kind: bool = high_count == 3;

        return is_three_of_kind;
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

        let is_full_house: bool = triplet_count == 1 && pair_count == 1;

        return is_full_house;
    }

    fn is_two_pair(hand: &str) -> bool {
        let (counts, _high_count): (HashMap<char, u8>, u8) = count_cards(hand);
        let mut pair_count: u8 = 0;

        for (_card, count) in counts {
            if count == 2 {
                pair_count += 1;
            }
        }

        let is_two_pair: bool = pair_count == 2;

        return is_two_pair;
    }

    // if highest card count is 2, all other cards must be different
    fn is_single_pair(hand: &str) -> bool {
        let (counts, _high_count): (HashMap<char, u8>, u8) = count_cards(hand);

        let mut pair_count: u8 = 0;
        let mut triplet_count: u8 = 0;

        for (_card, count) in counts.iter() {
            if *count == 2 {
                pair_count += 1;
            } else if *count == 3 {
                triplet_count += 1;
            }
        }

        let is_single_pair: bool = pair_count == 1 && triplet_count == 0;

        return is_single_pair;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand<'a> {
    hand: &'a str,
    bid: u32,
    rank: Rank,
}

impl Hand<'_> {
    pub fn compare(
        &self, other: &Self, card_ranks: &HashMap<char, u8>) -> Ordering {
        if self.hand == other.hand {
            return Ordering::Equal;
        }

        if self.beats(other, card_ranks) {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
    pub fn beats(&self, other: &Hand, card_ranks: &HashMap<char, u8>) -> bool {
        if self.rank == other.rank {
            return self.is_hand_stronger(other, card_ranks);
        } else {
            return self.rank > other.rank;
        }
    }

    // check if hand is stronger than the other hand if they
    // both share the same rank (i.e. both hands contain 2 pairs)
    fn is_hand_stronger(
        &self, other: &Hand, card_ranks: &HashMap<char, u8>) -> bool {

        for (self_card, other_card) in
            self.hand.chars().zip(other.hand.chars()) {
            if self_card != other_card {
                return card_ranks[&self_card] > card_ranks[&other_card];
            }
        }

        assert!(self.hand != other.hand);
        return false;
    }
}

pub fn get_card_ranks_puzzle1() -> HashMap<char, u8> {
    let rank_map: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    return rank_map;
}

pub fn get_card_ranks_puzzle2() -> HashMap<char, u8> {
    let rank_map: HashMap<char, u8> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    return rank_map;
}

// parse the hand of cards from the current line and determine its rank
pub fn parse_hand<'a> (line: &'a str, with_joker: bool) -> Hand<'a> {
    const DELIMITER: &str = " ";

    assert!(line.find(DELIMITER).is_some());
    let delimiter_idx: usize = line.find(DELIMITER).unwrap();

    let hand: &str = &line[0..delimiter_idx];
    assert!(is_valid_hand(hand));

    let bid: u32 = (&line[delimiter_idx+1..]).parse().unwrap();
    let rank: Rank = get_rank(hand, with_joker);

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

fn get_rank(hand: &str, with_joker: bool) -> Rank {
    let rank: Rank = if Rank::is_five_of_kind(hand) {
        Rank::FiveOfKind
    } else if Rank::is_four_of_kind(hand) {
        Rank::FourOfKind
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

    if with_joker {
        let upgraded_rank: Rank = try_to_upgrade_rank(&rank, hand);
        return upgraded_rank;
    }

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

// figure out how many free joker cards are available in the current hand
// to make a better hand
fn get_free_joker_count(card_counts: &HashMap<char, u8>, rank: &Rank) -> u8 {
    const JOKER_CARD: &char = &'J';
    let joker_count: Option<&u8> = card_counts.get(JOKER_CARD);

    // count the number of jokers
    let joker_count: u8 = if joker_count.is_some() {
        *joker_count.unwrap()
    } else {
        return 0;
    };

    let free_joker_count: u8 = match rank {
        // no free jokers available
        Rank::FiveOfKind => 0,

        // either 4 jokers form the rank, or 1 free joker is available;
        // either way, this can be turned into a FiveOfKind
        Rank::FourOfKind => joker_count,

        // if there are any joker cards,
        // either 3 or 2 jokers are part of the full house
        // and can be used to make a FiveOfKind
        Rank::FullHouse => joker_count,

        // if 3 jokers form the rank, we can use 1 free joker to turn
        // the ThreeOfKind into a FourOfKind;
        Rank::ThreeOfKind => if joker_count == 3 {1} else {joker_count},

        // if there are two pairs, we can have at most 1 or 2 joker cards,
        // which can be used to turn the pair into a FullHouse or a FourOfKind
        Rank::TwoPair => joker_count,

        // if two jokers from the rank, we can use them to form a ThreeOfKind
        Rank::SinglePair => if joker_count == 2 {1} else {joker_count},

        // hand can contain at most 1 joker here,
        // so it can be used to from a SinglePair
        Rank::HighCard => joker_count,  // will be 1 here
    };

    return free_joker_count;
}

// check if the rank of the hand can be upgraded
// with the use of available joker cards
fn try_to_upgrade_rank(rank: &Rank, hand: &str) -> Rank {
    let (card_counts, _): (HashMap<char, u8>, _) = count_cards(hand);
    let mut free_joker_count: u8 = get_free_joker_count(&card_counts, &rank);
    let mut upgraded_rank: Rank = rank.clone();

    while free_joker_count > 0 {
        upgraded_rank = upgrade_rank(&upgraded_rank);
        free_joker_count -= 1;
    }

    assert!(upgraded_rank >= *rank);

    return upgraded_rank;
}

// upgrade a rank to the next higher rank
// (assumes that there is at least one available joker card to do so)
fn upgrade_rank(rank: &Rank) -> Rank {
    let upgraded_rank: Rank = match rank {
        Rank::FiveOfKind | Rank::FourOfKind => Rank::FiveOfKind,
        Rank::FullHouse => Rank::FourOfKind,
        Rank::ThreeOfKind => Rank::FourOfKind,
        Rank::TwoPair => Rank::FullHouse,
        Rank::SinglePair => Rank::ThreeOfKind,
        Rank::HighCard => Rank::SinglePair,
    };

    return upgraded_rank;
}
