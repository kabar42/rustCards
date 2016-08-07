use card::*;


#[allow(dead_code)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let c: Vec<Card> = Vec::with_capacity(64);
        let d: Deck = Deck{cards: c};
        d
    }

    pub fn copy(other: &Deck) -> Deck {
        let mut c: Vec<Card> = Vec::with_capacity(64);

        if other.len() != 0 {
            for card in other.cards.iter() {
                c.push(Card{suit: card.suit, rank: card.rank});
            }
        }

        let d: Deck = Deck{cards: c};
        d
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn append(&mut self, c: Card) {
        self.cards.push(Card{suit: c.suit, rank: c.rank});
    }

    pub fn remove(&mut self, c: Card) {
        let (found, index) = self.find_card(c);
        if found && self.len() == 1 {
            self.cards.clear()
        } else if found {
            self.cards.remove(index); 
        }
    }

    fn find_card(&mut self, c: Card) -> (bool, usize) {
        let mut found: bool = false;
        let mut index: usize = 0;

        if self.len() > 0 {
            for i in 0..(self.len()-1) {
                if c ==  self.cards[i] {
                    found = true;
                    index = i;
                    break;
                }
            }
        }

        (found, index)
    }

}

pub fn build_std_deck() -> Deck {
    let mut deck: Deck = Deck::new();

    for s in Suit::iter() {
        for r in Rank::iter() {
            // println!("Got rank {:?}, suit {:?}", r, s);
            deck.append(Card{suit: *s, rank: *r})
        }
    }

    deck
}
