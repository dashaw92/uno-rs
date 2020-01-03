use crate::deck::Deck;
use crate::direction::*;
use crate::player::Player;

pub struct Uno {
    draw_deck: Deck,
    discard: Deck,
    direction: GameDirection,
    current_turn: usize,
    players: Vec<Player>,
}

impl Uno {
    pub fn create_game(max_players: usize) -> Uno {
        Uno {
            draw_deck: Deck::default(),
            discard: Vec::with_capacity(108).into(),
            direction: GameDirection::Clockwise,
            current_turn: 0,
            players: Vec::with_capacity(max_players),
        }
    }
}