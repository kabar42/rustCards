use card::*;

pub fn build_std_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::with_capacity(52);
    for s in Suit::iter() {
        for r in Rank::iter() {
            deck.push(Card{suit: *s, rank: *r})
        }
    }
    deck
}
