use self::Suit::*;
use self::Rank::*;

use std::slice::Iter;

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

