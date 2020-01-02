#![allow(unused)]

pub trait Card {
    fn can_play_on(&self, other: &CardType) -> bool;
}

impl Card for WildCard {
    fn can_play_on(&self, other: &CardType) -> bool {
        true
    }
}

impl Card for ColorCard {
    fn can_play_on(&self, other: &CardType) -> bool {
        match other {
            CardType::Wild(card) => {
                match card.face {
                    WildFace::DrawFour => true,
                    WildFace::ColorWild(color) => self.color == color
                }
            },
            CardType::Colored(card) => self.face == card.face || self.color == card.color
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum CardType {
    Colored(ColorCard),
    Wild(WildCard)
}

#[derive(PartialEq, Eq)]
pub struct WildCard {
    face: WildFace,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WildFace {
    DrawFour,
    ColorWild(Color),
}

#[derive(PartialEq, Eq)]
pub struct ColorCard {
    color: Color,
    face: Face,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Face {
    DrawTwo,
    Skip,
    Reverse,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}