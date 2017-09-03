use std::fmt;
use std::slice::Iter;

use self::Suit::*;
use self::Rank::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades
}

pub const SUIT_COUNT: usize = 4;

impl Suit {
    pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit; SUIT_COUNT] = [ Hearts, Clubs, Diamonds, Spades ];
        SUITS.into_iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str;
        match *self {
            Hearts => s = "H",
            Clubs => s = "C",
            Diamonds => s = "D",
            Spades => s = "S",
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    InvalidRank
}

pub const RANK_COUNT: usize = 13;

impl Rank {
    pub fn iter() -> Iter<'static, Rank> {
        static RANKS: [Rank; RANK_COUNT] = [ Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King ];
        RANKS.into_iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str;
        match *self {
            Ace => s = "A",
            Two => s = "2",
            Three => s = "3",
            Four => s = "4",
            Five => s = "5",
            Six => s = "6",
            Seven => s = "7",
            Eight => s = "8",
            Nine => s = "9",
            Ten => s = "10",
            Jack => s = "J",
            Queen => s = "Q",
            King => s = "K",
            InvalidRank => s = "ERR",
        }
        write!(f, "{}", s)
    }
}

pub fn int_to_rank(value: usize) -> Rank {
    match value {
        0 => Rank::Ace,
        1 => Rank::Two,
        2 => Rank::Three,
        3 => Rank::Four,
        4 => Rank::Five,
        5 => Rank::Six,
        6 => Rank::Seven,
        7 => Rank::Eight,
        8 => Rank::Nine,
        9 => Rank::Ten,
        10 => Rank::Jack,
        11 => Rank::Queen,
        12 => Rank::King,
        _ => Rank::InvalidRank,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.suit == other.suit &&
           self.rank == other.rank {
            return true
        } else {
            return false
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

