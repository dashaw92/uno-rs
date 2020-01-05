use crate::card::Card;
use crate::deck::Deck;

use std::fmt::{self, Debug, Display};

pub struct Player {
    pub name: String,
    pub hand: Deck,
}

impl Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player: {}, Hand: {}", self.name, self.hand)
    }
}

impl Player {
    pub fn new<S: ToString>(name: S) -> Player {
        Player {
            name: name.to_string(),
            //We'll assume no player will get more than this many cards.
            //If they do.... unlucky :^)
            hand: Vec::with_capacity(25).into(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
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