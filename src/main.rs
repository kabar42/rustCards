mod card;
mod hand;
mod deck;
mod analyze_hands;

use hand::*;
use deck::*;
use analyze_hands::*;

fn main() {
    let deck = build_std_deck();
    let hand = Hand::new(5);
    let mut hands: Vec<Hand> = Vec::with_capacity(3000000);

    gen_all_hands(&deck, hand, &mut hands);
    println!("Hands: {}", hands.len());

    let type_counts = count_hand_types(hands);
    for (hand_type, count) in type_counts.iter().enumerate() {
        println!("{}: {}", hand_type, *count);
    }
}
