mod card;
mod hand;
mod deck;

use card::*;
use hand::*;
use deck::*;

fn main() {
    let deck: Vec<Card> = build_std_deck();
    println!("Cards in deck: {}", deck.len());
    let mut hand: Hand = Hand::new(5);
    let mut hands: Vec<Hand> = Vec::with_capacity(3000000);

    gen_all_hands(&deck, &mut hand, &mut hands);
    println!("Hands: {}", hands.len());

    return
}
