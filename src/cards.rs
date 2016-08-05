use self::Suit::*;
use self::Rank::*;
use std::slice::Iter;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
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

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
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

pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut c: Vec<Card> = vec![];
        let mut d: Deck = Deck{cards: c};
        d
    }

    pub fn append(&mut self, c: Card) {
        self.cards.push(Card{suit: c.suit, rank: c.rank});
    }
}
