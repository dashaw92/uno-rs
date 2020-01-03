use crate::deck::Deck;
use crate::direction::*;
use crate::player::Player;

use std::fmt::{self, Display};

pub struct Uno {
    draw_deck: Deck,
    discard: Deck,
    direction: GameDirection,
    current_turn: usize,
    players: Vec<Player>,
}

impl Uno {
    pub fn create_game(players: Vec<Player>) -> Uno {
        if players.len() < 2 {
            panic!("Need at least two players to play!");
        }

        if players.len() > 4 {
            panic!("Cannot have more than 4 players in a game!");
        }

        let mut uno = Uno {
            draw_deck: Deck::default(),
            discard: Vec::with_capacity(108).into(),
            direction: GameDirection::Clockwise,
            current_turn: 0,
            players,
        };

        //this method of handing out cards is technically not fair
        //since players will get cards from (n * players) -> (n * players + 7) cards,
        //when it should cycle between all players for each card
        //
        //but who cares
        (0..uno.players.len()).into_iter().for_each(|i| {
            (0..7).into_iter().for_each(|_| {
                let card = match uno.draw_deck.draw() {
                    Some(card) => card,
                    None => {
                        uno.draw_deck.reclaim(&mut uno.discard);
                        //should never happen, but whatever
                        //if it's an issue, I can always revist it
                        uno.draw_deck.draw().unwrap()
                    }
                };

                uno.players[i].add_card(card);
            });
        });

        uno.discard += uno.draw_deck.draw().unwrap();
        uno
    }
}

impl Display for Uno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let players = self.players.iter().map(|p| format!("{} ", p)).fold(String::new(), |acc, p| acc + p.as_ref());
        
        write!(f, "Cards in draw pile: {}
Cards in discard: {}
Top discard: {:?}
Players: {}
Direction: {:?}
Turns: {}",
                (*self.draw_deck).len(), 
                (*self.discard).len(), 
                self.discard.peek_top_card(), 
                players,
                self.direction,
                self.current_turn)
    }
}