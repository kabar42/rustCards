use self::Suit::*;
use self::Rank::*;

use std::slice::Iter;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Hearts,
    Clubs,
    Diamonds,
    Spades
}

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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

