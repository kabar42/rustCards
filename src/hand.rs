use card::*;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub max_size: usize
}

impl Hand {
    pub fn new(size: usize) -> Hand {
        let c: Vec<Card> = Vec::with_capacity(size);
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

    pub fn append(&mut self, c: Card) {
        if self.cards.len() < self.max_size {
            self.cards.push(Card{suit: c.suit, rank: c.rank});
        } else {
            panic!("Cannot add card to hand. It is already full.");
        }
    }

    pub fn full(&self) -> bool {
        if self.cards.len() >= self.max_size {
            return true
        } else {
            return false
        }
    }
}

pub fn gen_all_hands(deck: &[Card], hand: Hand, mut hands: &mut Vec<Hand>) {
    if hand.full() {
        hands.push(hand);
    } else if deck.len() > 0 {
        let mut deck_slice: &[Card] = &Vec::with_capacity(0);
        if deck.len() > 0 {
            deck_slice = &deck[0..deck.len()-1];
        }
        {
            let mut new_hand: Hand = Hand::copy(&hand);
            new_hand.append(deck[0]);
            gen_all_hands(deck_slice, new_hand, &mut hands);
        }
        gen_all_hands(deck_slice, hand, &mut hands);
    }
}

