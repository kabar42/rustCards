use std::fmt;

use card::*;
use hand::*;

use self::HandType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandType {
    NoPair,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush
}

const HAND_TYPES_COUNT: usize = 10;

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str;
        match *self {
            NoPair          => s = "No Pair",
            OnePair         => s = "One Pair",
            TwoPair         => s = "Two Pair",
            ThreeOfAKind    => s = "Three Of A Kind",
            Straight        => s = "Straight",
            Flush           => s = "Flush",
            FullHouse       => s = "Full House",
            FourOfAKind     => s = "Four Of A Kind",
            StraightFlush   => s = "Straight Flush",
            RoyalFlush      => s = "Royal Flush",
        }
        write!(f, "{}", s)
    }
}

pub struct HandData {
    pub suit_count: [i32; SUIT_COUNT],
    pub rank_count: [i32; RANK_COUNT]
}

pub fn count_hand_types(all_hands: Vec<Hand>) -> Vec<i32> {
    let mut type_counts = vec![0; HAND_TYPES_COUNT];

    for hand in all_hands.iter() {
        let data = get_hand_data(hand);
        let hand_type = determine_hand_type(data);
        type_counts[hand_type as usize] += 1;
    }
    return type_counts
}

fn get_hand_data(hand: &Hand) -> HandData {
    let mut data = HandData{suit_count: [0; SUIT_COUNT], rank_count: [0; RANK_COUNT]};
    for card in hand.cards.iter() {
        data.suit_count[card.suit as usize] += 1;
        data.rank_count[card.rank as usize] += 1;
    }
    return data
}

fn determine_hand_type(data: HandData) -> HandType {
    let mut largest_rank_count = 0;
    let mut second_largest_rank_count = 0;

    for count in data.rank_count.iter() {
        if *count > largest_rank_count {
            second_largest_rank_count = largest_rank_count;
            largest_rank_count = *count;
        } else if *count > second_largest_rank_count {
            second_largest_rank_count = *count;
        }
    }

    let mut ranks_present = get_ranks_present(&data.rank_count);

    if array_contains(&data.suit_count, HAND_SIZE) {
        if is_royal_flush(&data.rank_count) {
            return HandType::RoyalFlush
        }

        if ranks_are_sequential(&mut ranks_present) {
            return HandType::StraightFlush
        }

        return HandType::Flush
    }

    if array_contains(&data.rank_count, SUIT_COUNT) {
        return HandType::FourOfAKind
    }

    if ranks_are_sequential(&mut ranks_present) {
        return HandType::Straight
    }

    if largest_rank_count == 3 {
        if second_largest_rank_count == 2 {
            return HandType::FullHouse
        }
        return HandType::ThreeOfAKind
    }

    if largest_rank_count == 2 {
        if second_largest_rank_count == 2 {
            return HandType::TwoPair
        }
        return HandType::OnePair
    }

    return HandType::NoPair
}

fn get_ranks_present(counts: &[i32]) -> Vec<Rank> {
    let mut ranks: Vec<Rank> = Vec::with_capacity(RANK_COUNT);
    for i in 0..counts.len() {
        if counts[i] > 0 {
            ranks.push(int_to_rank(i));
        }
    }
    return ranks
}

fn array_contains(counts: &[i32], val: usize) -> bool {
    for v in counts.iter() {
        if *v == val as i32 {
            return true;
        }
    }
    return false;
}

fn is_royal_flush(rank_count: &[i32]) -> bool {
    if rank_count[Rank::Ten as usize] == 1 &&
        rank_count[Rank::Jack as usize] == 1 &&
        rank_count[Rank::Queen as usize] == 1 &&
        rank_count[Rank::King as usize] == 1 &&
        rank_count[Rank::Ace as usize] == 1 {
        return true;
    }
    return false;
}

fn ranks_are_sequential(ranks: &mut Vec<Rank>) -> bool {
    if ranks.len() < HAND_SIZE {
        return false
    }

    ranks.sort();

    if ranks[0] == Rank::Ace &&
       ranks[1] == Rank::Ten &&
       ranks[2] == Rank::Jack &&
       ranks[3] == Rank::Queen &&
       ranks[4] == Rank::King {
        return true
    }

    for i in 1..ranks.len() {
        let prev_rank = ranks[i-1] as i32;
        let this_rank = ranks[i] as i32;
        if (ranks[i-1] != Rank::InvalidRank) &&
           (ranks[i] != Rank::InvalidRank) &&
           prev_rank != this_rank-1 {
            return false
        }
    }
    
    return true
}
