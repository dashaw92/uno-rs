use crate::card::CardType;
use crate::deck::Deck;

pub struct Player {
    name: String,
    hand: Deck,
}

impl Player {
    pub fn new<S: ToString>(name: S) -> Player {
        Player {
            name: name.to_string(),
            //Reserve space for half the deck. 
            //We'll assume no player will get more than that.
            //If they do.... unlucky :^)
            hand: Vec::with_capacity(54).into(),
        }
    }

    pub fn add_card(&mut self, card: CardType) {
        self.hand += card;
    }
}