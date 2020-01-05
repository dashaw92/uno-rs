use crate::card::{*, face::*, color::*};

use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut, AddAssign, SubAssign};

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        shuffle::shuffle(&mut self.cards);
    }

    pub fn draw(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }

        Some(self.cards.remove(self.cards.len() - 1))
    }

    pub fn peek_top_card(&self) -> Option<&Card> {
        if self.cards.is_empty() {
            return None;
        }

        self.cards.get(0)
    }

    pub fn reclaim(&mut self, other: &mut Deck) {
        (*self).append(&mut (*other));
    }

    pub fn has_card(&self, rhs: Card) -> bool {
        self.cards.iter().any(|&card| card == rhs)
    }
}

impl Default for Deck {
    fn default() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(108);

        &[Color::Red, Color::Green, Color::Blue, Color::Yellow].into_iter().for_each(|&color| {
            cards.push(Card::new(color, Face::from('0').unwrap()).into());
            (0..2).into_iter().for_each(|_| {
                cards.push(Card::new(color, Face::DrawTwo).into());
                cards.push(Card::new(color, Face::Reverse).into());
                cards.push(Card::new(color, Face::Skip).into());

                (1..=9).into_iter().for_each(|val| {
                    let ch = match val {
                        1 => '1',
                        2 => '2',
                        3 => '3',
                        4 => '4',
                        5 => '5',
                        6 => '6',
                        7 => '7',
                        8 => '8',
                        9 => '9',
                        _ => unreachable!(),
                    };
                    cards.push(Card::new(color, Face::from(ch).unwrap()).into())
                });
            });
        });

        (0..4).into_iter().for_each(|_| {
            cards.push(Card::new(Color::Red, Face::DrawFour));
            cards.push(Card::new(Color::Red, Face::ColorCard));
        });

        let mut deck = Deck::new(cards);
        deck.shuffle();
        deck
    }
}

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let joined: Vec<_> = self.cards.iter()
            .map(Card::display_name)
            .collect();
        write!(f, "{:?}", joined)
    }
}

impl From<Vec<Card>> for Deck {
    fn from(vec: Vec<Card>) -> Deck {
        Deck::new(vec)
    }
}

impl AddAssign<Card> for Deck {
    fn add_assign(&mut self, rhs: Card) {
        (*self).insert(0, rhs);
    }
}

impl SubAssign<Card> for Deck {
    fn sub_assign(&mut self, rhs: Card) {
        for i in 0..self.cards.len() {
            if (*self)[i] == rhs {
                (*self).remove(i);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut deck = Deck::default();
        let top = deck.draw().unwrap();
        assert_eq!(107, (*deck).len());

        deck += top.clone();
        assert_eq!(deck.peek_top_card(), Some(&top));
        assert_eq!(108, (*deck).len());
    }

    #[test]
    fn test_sub_assign() {
        let mut deck = Deck::default();
        let top = deck.peek_top_card().unwrap().clone();
        assert_eq!(108, (*deck).len());

        deck -= top;
        assert_eq!(107, (*deck).len());
    }
}