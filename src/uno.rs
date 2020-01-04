use crate::card::{*, face::*, color::*};
use crate::deck::Deck;
use crate::direction::*;
use crate::player::Player;

pub struct Uno {
    pub draw_deck: Deck,
    pub discard: Deck,
    pub direction: GameDirection,
    pub current_turn: usize,
    pub players: Vec<Player>,
    pub current_player: usize,
}

impl Uno {
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
        current = match current as isize + self.direction {
            x if x < 0 => self.players.len() - 1,
            x => x as usize,
        };
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
                let top = (*self.discard).remove(0);
                if !(*self.discard).is_empty() {
                    self.draw_deck.reclaim(&mut self.discard);
                    self.discard += top;
                    self.draw_card()
                } else {
                    self.draw_deck.reclaim(&mut Deck::default());
                    self.discard += top;
                    self.draw_card()
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TurnResult {
    Success(Card),
    InvalidMove(Card, Card),
    NotHoldingCard(Card),
}