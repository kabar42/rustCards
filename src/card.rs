use std::fmt;
use std::slice::Iter;

use self::Suit::*;
use self::Rank::*;

#[allow(dead_code)]
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
        static SUITS: [Suit; 4] = [ Hearts, Clubs, Diamonds, Spades ];
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

#[allow(dead_code)]
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
    King
}

pub const RANK_COUNT: usize = 13;

impl Rank {
    pub fn iter() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [ Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King ];
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
        }
        write!(f, "{}", s)
    }
}

pub fn int_to_rank(value: usize) -> Rank {
    match value {
        1 => Rank::Ace,
        2 => Rank::Two,
        3 => Rank::Three,
        4 => Rank::Four,
        5 => Rank::Five,
        6 => Rank::Six,
        7 => Rank::Seven,
        8 => Rank::Eight,
        9 => Rank::Nine,
        10 => Rank::Ten,
        11 => Rank::Jack,
        12 => Rank::Queen,
        13 => Rank::King,
        _ => Rank::Two,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.suit == other.suit && self.rank == other.rank {
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

