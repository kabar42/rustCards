mod card;
mod hand;
mod deck;

use card::*;
use hand::*;
use deck::*;

use std::thread;

fn main() {
    let child = thread::Builder::new().stack_size(32*1024*1024).spawn(move || {
        let deck: Box<Deck> = Box::new(build_std_deck());

        let hand: Box<Hand> = Box::new(Hand::new(5));
        let mut hands: Vec<Hand> = Vec::with_capacity(500000);

        gen_all_hands(&deck, &hand, &mut hands);

        // for hand in hands.iter() {
        //     println!("{:?}", hand);
        // }

        println!("Hands: {}", hands.len());

        return
    }).unwrap();

    child.join().unwrap();
}
