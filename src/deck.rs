use crate::card::*;

use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut, AddAssign, SubAssign};

pub struct Deck {
    cards: Vec<CardType>,
}

impl Deck {
    pub fn new(cards: Vec<CardType>) -> Self {
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        shuffle::shuffle(&mut self.cards);
    }

    pub fn draw(&mut self) -> Option<CardType> {
        if self.cards.is_empty() {
            return None;
        }

        Some(self.cards.remove(self.cards.len() - 1))
    }

    pub fn peek_top_card(&self) -> Option<&CardType> {
        if self.cards.is_empty() {
            return None;
        }

        self.cards.get(0)
    }

    pub fn reclaim(&mut self, other: &mut Deck) {
        (*self).append(&mut (*other));
    }

    pub fn has_card(&self, card: CardType) -> bool {
        for i in 0..self.cards.len() {
            match card {
                CardType::Wild(WildCard { face: WildFace::ColorWild(_) }) => {
                    if let CardType::Wild(WildCard { face: WildFace::ColorWild(_)}) = (*self)[i] {
                        return true;
                    }
                },
                _ => if (*self)[i] == card {
                    return true;
                },
            }
        }

        false
    }
}

impl Default for Deck {
    fn default() -> Deck {
        let mut cards: Vec<CardType> = Vec::with_capacity(108);

        &[Color::Red, Color::Green, Color::Blue, Color::Yellow].into_iter().for_each(|&color| {
            cards.push(ColorCard::new(color, 0.into()).into());
            (0..2).into_iter().for_each(|_| {
                cards.push(ColorCard::new(color, Face::DrawTwo).into());
                cards.push(ColorCard::new(color, Face::Reverse).into());
                cards.push(ColorCard::new(color, Face::Skip).into());

                (1..=9).into_iter().for_each(|val| cards.push(ColorCard::new(color, val.into()).into()));
            });
        });

        (0..4).into_iter().for_each(|_| {
            cards.push(WildCard::new(WildFace::DrawFour).into());
            cards.push(WildCard::new(WildFace::ColorWild(Color::Red)).into());
        });

        let mut deck = Deck::new(cards);
        deck.shuffle();
        deck
    }
}

impl Deref for Deck {
    type Target = Vec<CardType>;

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
        let joined = self.cards.iter().map(|card| format!(" {} :", card.display_name())).fold(String::new(), |acc, card| acc + &card);
        write!(f, "{}", joined)
    }
}

impl From<Vec<CardType>> for Deck {
    fn from(vec: Vec<CardType>) -> Deck {
        Deck::new(vec)
    }
}

impl AddAssign<CardType> for Deck {
    fn add_assign(&mut self, rhs: CardType) {
        (*self).insert(0, rhs);
    }
}

impl SubAssign<CardType> for Deck {
    fn sub_assign(&mut self, rhs: CardType) {
        if !self.contains(&rhs) {
            return;
        }

        for i in 0..self.cards.len() {
            match rhs {
                CardType::Wild(WildCard { face: WildFace::ColorWild(_) }) => {
                    if let CardType::Wild(WildCard { face: WildFace::ColorWild(_)}) = (*self)[i] {
                        (*self).remove(i);
                        return;
                    }
                },
                _ => if (*self)[i] == rhs {
                    (*self).remove(i);
                    return;
                },
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