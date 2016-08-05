mod cards;

fn main() {

    let mut deck: cards::Deck = cards::Deck::new();

    for s in cards::Suit::iter() {
        for r in cards::Rank::iter() {
            println!("Got rank {:?}, suit {:?}", r, s);
            deck.append(cards::Card{suit: *s, rank: *r})
        }
    }
}
