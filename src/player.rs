use crate::card::CardType;
use crate::deck::Deck;

use std::fmt::{self, Display};

pub struct Player {
    pub name: String,
    pub hand: Deck,
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

impl From<&str> for Player {
    fn from(name: &str) -> Player {
        Player::new(name)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}