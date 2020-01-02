mod card;
mod deck;

use crate::deck::Deck;
// use crate::card::*;

fn main() {
    let deck = Deck::default();
    let cards = &*deck;
    for card in cards {
        println!("{}", card);
    }
}