use card::*;
use deck::*;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize
}

impl Hand {
    pub fn new(size: usize) -> Hand {
        let c: Vec<Card> = Vec::with_capacity(5);
        let h: Hand = Hand{ cards: c, max_size: size };
        h
    }

    pub fn copy(other: &Hand) -> Hand {
        let mut c: Vec<Card> = Vec::with_capacity(5);
        for card in other.cards.iter() {
            c.push(Card{suit: card.suit, rank: card.rank})
        }

        let h: Hand = Hand{cards: c, max_size: other.max_size};
        h
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn append(&mut self, c: Card) {
        if self.len() < self.max_size {
            self.cards.push(Card{suit: c.suit, rank: c.rank});
        } else {
            panic!("Cannot add card to hand. It is already full.");
        }
    }

    pub fn full(&self) -> bool {
        if self.len() >= self.max_size {
            return true
        } else {
            return false
        }
    }
}

pub fn gen_all_hands(deck: &Deck, hand: &Hand, mut hands: &mut Vec<Hand>) {

    if hand.full() {
        let new_hand: Hand = Hand::copy(&hand);
        hands.push(new_hand);
    } else if deck.len() > 0 {
        let mut deck_copy: Deck = Deck::copy(&deck);
        let card: Card = deck_copy.cards.remove(0);

        {
            let mut new_hand: Hand = Hand::copy(&hand);
            new_hand.append(card);
            gen_all_hands(&deck_copy, &new_hand, &mut hands);
        }

        gen_all_hands(&deck_copy, &hand, &mut hands);
    }
}

