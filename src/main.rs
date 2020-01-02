mod card;
mod deck;

use crate::deck::Deck;
// use crate::card::*;

fn main() {
    let mut deck = Deck::default();
    deck.shuffle();
    let cards = &*(deck);
    for card in cards {
        println!("{}", card);
    }
}