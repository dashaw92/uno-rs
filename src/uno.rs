use crate::card::{*, face::*, color::*};
use crate::deck::Deck;
use crate::direction::*;
use crate::player::Player;

use std::fmt::{self, Display};

pub struct Uno {
    pub draw_deck: Deck,
    pub discard: Deck,
    pub direction: GameDirection,
    current_turn: usize,
    players: Vec<Player>,
    current_player: usize,
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
            current_turn: 1,
            players,
            current_player: 0,
        };

        //this method of handing out cards is technically not fair
        //since players will get cards from (n * players) -> (n * players + 7) cards,
        //when it should cycle between all players for each card
        //
        //but who cares
        (0..uno.players.len()).into_iter().for_each(|i| {
            (0..7).into_iter().for_each(|_| {
                let card = uno.draw_card();
                uno.players[i].add_card(card);
            });
        });

        let top = uno.draw_card();
        uno.discard += top;
        uno
    }

    pub fn play_card(&mut self, card: Card) -> TurnResult {
        let top_discard = match self.discard.peek_top_card() {
            Some(&card) => card,
            None => Card::new(Color::Red, Face::ColorCard).into(),
        };

        let player = &mut self.players[self.current_player];

        if !player.hand.has_card(card) {
            return TurnResult::NotHoldingCard(card);
        }

        if !card.can_play_on(top_discard) {
            return TurnResult::InvalidMove(top_discard, card);
        }

        player.hand -= card;
        self.discard += card;

        match card.face {
            Face::DrawFour | Face::DrawTwo => {
                let amount = match card.face {
                    Face::DrawFour => 4,
                    Face::DrawTwo => 2,
                    _ => unreachable!(),
                };

                self.do_turn_increase();
                
                //work around borrowing &mut self multiple times
                let drawcards: Vec<_> = (0..amount).into_iter().map(|_| self.draw_card()).collect();
                let player = &mut self.players[self.current_player];
                drawcards.into_iter().for_each(|card| player.add_card(card));
            },
            Face::Reverse => {
                self.direction = !self.direction;
                if self.players.len() < 3 {
                    self.do_turn_increase();
                }
            },
            Face::Skip => { self.do_turn_increase(); }, //skip next player
            _ => {},
        }

        self.do_turn_increase();
        TurnResult::Success(card)
    }

    fn do_turn_increase(&mut self) -> usize {
        self.current_turn += 1;
        let mut current = self.current_player;
        current += self.direction;
        current %= self.players.len();
        self.current_player = current;
        self.current_player
    }

    pub fn current_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    pub fn draw_card(&mut self) -> Card {
        match self.draw_deck.draw() {
            Some(card) => card,
            None => {
                self.draw_deck.reclaim(&mut self.discard);
                match self.draw_deck.draw() {
                    Some(card) => {
                        self.discard += card;
                        self.draw_deck.draw().unwrap()
                    },
                    None => {
                        self.draw_deck.reclaim(&mut Deck::default());
                        self.discard += self.draw_deck.draw().unwrap();
                        self.draw_deck.draw().unwrap()
                    }
                }
            }
        }
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

#[derive(Debug)]
pub enum TurnResult {
    Success(Card),
    InvalidMove(Card, Card),
    NotHoldingCard(Card),
}