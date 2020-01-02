#![allow(unused)]

pub trait Card {
    fn can_play_on(&self, other: &CardType) -> bool;
}

impl WildCard {
    pub fn new(face: WildFace) -> Self {
        WildCard { face }
    }
}

impl Card for WildCard {
    fn can_play_on(&self, other: &CardType) -> bool {
        true
    }
}

impl From<ColorCard> for CardType {
    fn from(other: ColorCard) -> CardType {
        CardType::Colored(other)
    }
}

impl From<WildCard> for CardType {
    fn from(other: WildCard) -> CardType {
        CardType::Wild(other)
    }
}

impl ColorCard {
    pub fn new(color: Color, face: Face) -> Self {
        ColorCard { color, face }
    }
}

impl Card for ColorCard {
    fn can_play_on(&self, other: &CardType) -> bool {
        match other {
            CardType::Wild(card) => {
                match card.face {
                    WildFace::DrawFour => true,
                    WildFace::ColorWild(color) => self.color == color,
                }
            },
            CardType::Colored(card) => self.face == card.face || self.color == card.color
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CardType {
    Colored(ColorCard),
    Wild(WildCard)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WildCard {
    pub face: WildFace,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WildFace {
    DrawFour,
    ColorWild(Color),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ColorCard {
    pub color: Color,
    pub face: Face,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

impl From<isize> for Face {
    fn from(input: isize) -> Face {
        match input {
            0 => Face::Zero,
            1 => Face::One,
            2 => Face::Two,
            3 => Face::Three,
            4 => Face::Four,
            5 => Face::Five,
            6 => Face::Six,
            7 => Face::Seven,
            8 => Face::Eight,
            9 => Face::Nine,
            _ => panic!("Cannot create a face with a value outside the range of 0-9!"),
        }
    }
}

impl From<Face> for isize {
    fn from(face: Face) -> isize {
        match face {
            Face::Zero => 0,
            Face::One => 1,
            Face::Two => 2,
            Face::Three => 3,
            Face::Four => 4,
            Face::Five => 5,
            Face::Six => 6,
            Face::Seven => 7,
            Face::Eight => 8,
            Face::Nine => 9,
            _ => panic!("Invalid face value."),
        }
    }
}