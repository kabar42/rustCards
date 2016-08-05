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

struct Card {
    suit: Suit,
    rank: Rank
}

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn New() -> Deck {
        let mut c: Vec<Card> = vec![];
        let mut d: Deck = Deck{cards: c};
        d
    }

    pub fn append(&mut self, c: Card) {
        self.cards.push(Card{suit: c.suit, rank: c.rank});
    }
}


fn main() {

    let mut deck: Deck = Deck::New();

    for s in Suit::iter() {
        for r in Rank::iter() {
            println!("Got rank {:?}, suit {:?}", r, s);
            deck.append(Card{suit: *s, rank: *r})
        }
    }
}
