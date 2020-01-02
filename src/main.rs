mod card;
mod deck;

use crate::deck::Deck;
// use crate::card::*;

fn main() {
    let deck = Deck::default();
    let cards = &*deck;
    cards.into_iter().for_each(|card| println!("{}", card));
}