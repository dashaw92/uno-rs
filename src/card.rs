pub trait Card {
    fn can_play_on<C: Into<CardType>>(&self, other: C) -> bool;
}

impl WildCard {
    pub fn new(face: WildFace) -> Self {
        WildCard { face }
    }
}

impl Card for WildCard {
    fn can_play_on<C: Into<CardType>>(&self, other: C) -> bool {
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
    fn can_play_on<C: Into<CardType>>(&self, other: C) -> bool {
        match other.into() {
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct WildCard {
    pub face: WildFace,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WildFace {
    DrawFour,
    ColorWild(Color),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_rules() {
        let draw_four = WildCard::new(WildFace::DrawFour);
        let draw_four2 = draw_four.clone();
        assert!(draw_four.can_play_on(draw_four));
        
        let green_draw_two = ColorCard::new(Color::Green, Face::DrawTwo);
        let red_draw_two = ColorCard::new(Color::Red, Face::DrawTwo);
        assert!(green_draw_two.can_play_on(red_draw_two));
        assert!(red_draw_two.can_play_on(green_draw_two));

        let yellow_zero = ColorCard::new(Color::Yellow, Face::Zero);
        let blue_nine = ColorCard::new(Color::Blue, Face::Nine);
        assert!(!yellow_zero.can_play_on(blue_nine));
        assert!(!blue_nine.can_play_on(yellow_zero));

        assert!(yellow_zero.can_play_on(draw_four));
        assert!(draw_four.can_play_on(yellow_zero))
    }
}