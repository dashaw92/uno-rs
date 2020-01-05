use crate::card::{color::*, face::*, *};
use crate::deck::Deck;
use crate::direction::*;
use crate::player::Player;

pub struct Uno {
    draw_deck: Deck,
    discard: Deck,
    direction: GameDirection,
    current_turn: usize,
    players: Vec<Player>,
    current_player: usize,
}

impl Uno {
    pub fn deck(&self) -> &Deck {
        &self.draw_deck
    }

    pub fn discard(&self) -> &Deck {
        &self.discard
    }

    pub fn direction(&self) -> GameDirection {
        self.direction
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn current_turn(&self) -> usize {
        self.current_turn
    }

    pub fn create_game(players: Vec<Player>) -> Uno {
        if players.len() < 2 {
            panic!("Need at least two players to play!");
        }

        if players.len() > 8 {
            panic!("Cannot have more than 8 players in a game!");
        }

        let mut uno = Uno {
            draw_deck: Deck::default(),
            discard: Vec::with_capacity(108).into(),
            direction: GameDirection::Clockwise,
            current_turn: 1,
            players,
            current_player: 0,
        };

        (0..7).into_iter().for_each(|_| {
            (0..uno.players.len()).into_iter().for_each(|i| {
                let card = uno.draw_card();
                uno.players[i].add_card(card);
            });
        });

        let top = uno.draw_card();
        uno.discard += top;
        uno
    }

    pub fn play_card(&mut self, card: Card) -> TurnResult {
        let top_discard = self
            .discard
            .peek_top_card()
            .map(|&x| x)
            .unwrap_or(Card::new(Color::Red, Face::ColorCard));
        let player = &mut self.players[self.current_player];

        if !player.get_hand().has_card(card) {
            return TurnResult::NotHoldingCard(card);
        }

        if !card.can_play_on(top_discard) {
            return TurnResult::InvalidMove(top_discard, card);
        }

        *player.get_hand_mut() -= card;
        self.discard += card;

        if player.get_hand().is_empty() {
            return TurnResult::GameOver;
        }

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
            }
            Face::Reverse => {
                self.direction = !self.direction;
                if self.players.len() < 3 {
                    self.do_turn_increase();
                }
            }
            Face::Skip => {
                self.do_turn_increase();
            } //skip next player
            _ => {}
        }

        self.do_turn_increase();
        TurnResult::Success(card)
    }

    fn do_turn_increase(&mut self) -> usize {
        self.current_turn += 1;
        self.current_player = match self.current_player as isize + self.direction {
            x if x < 0 => self.players.len() - 1,
            x => x as usize,
        };
        self.current_player %= self.players.len();
        self.current_player
    }

    pub fn current_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    pub fn draw_card(&mut self) -> Card {
        match self.draw_deck.draw() {
            Some(card) => card,
            None => {
                let top = (*self.discard).remove(0);
                if !(*self.discard).is_empty() {
                    self.draw_deck.reclaim(&mut self.discard);
                } else {
                    self.draw_deck.reclaim(&mut Deck::default());
                }

                self.discard += top;
                self.draw_card()
            }
        }
    }
}

#[derive(Debug)]
pub enum TurnResult {
    Success(Card),
    InvalidMove(Card, Card),
    NotHoldingCard(Card),
    GameOver,
}
