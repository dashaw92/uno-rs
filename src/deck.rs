use crate::card::*;

use std::fmt::{self, Display};
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

pub struct Deck {
    cards: Vec<CardType>,
}

impl Deck {
    fn new(cards: Vec<CardType>) -> Self {
        let mut deck = Deck { cards };

        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        self.cards.sort_by(random);
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

        Deck::new(cards)
    }
}

impl Deref for Deck {
    type Target = [CardType];

    fn deref(&self) -> &Self::Target {
        self.cards.as_slice()
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cards.as_mut_slice()
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CardType::Wild(wild) => {
                match wild.face {
                    WildFace::DrawFour => write!(f, "Draw Four"),
                    WildFace::ColorWild(color) => write!(f, "{:?} Wild Card", color),
                }
            },
            CardType::Colored(card) => {
                match card.face {
                    Face::DrawTwo => write!(f, "{:?} Draw Two", card.color),
                    Face::Skip => write!(f, "{:?} Skip", card.color),
                    Face::Reverse => write!(f, "{:?} Reverse", card.color),
                    face => {
                        let value: isize = face.into();
                        write!(f, "{:?} {}", card.color, value)
                    }
                }
            },
        }
    }
}

impl From<Vec<CardType>> for Deck {
    fn from(cards: Vec<CardType>) -> Deck {
        Deck::new(cards)
    }
}

fn random(_: &CardType, _: &CardType) -> Ordering {
    use std::time::SystemTime;

    let now = SystemTime::now();
    match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => {
            let ns = time.as_secs();
            match ns % 3 {
                0 => Ordering::Greater,
                1 => Ordering::Less,
                2 => Ordering::Equal,
                _ => Ordering::Greater,
            }
        },
        Err(_) => Ordering::Equal,
    }
}