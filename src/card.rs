use std::fmt::{self, Display};
use std::str::FromStr;

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

impl Card for CardType {
    fn can_play_on<C: Into<CardType>>(&self, other: C) -> bool {
        match self {
            CardType::Wild(wild) => wild.can_play_on(other),
            CardType::Colored(color) => color.can_play_on(other),
        }
    }
}

impl Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ctype = "";
        let mut face = "";
        let mut color = "";

        match self {
            CardType::Wild(wild) => {
                ctype = "W";
                match wild.face {
                    WildFace::DrawFour => face = "D",
                    WildFace::ColorWild(col) => {
                        face = "C";
                        color = match col {
                            Color::Red => "R",
                            Color::Green => "G",
                            Color::Blue => "B",
                            Color::Yellow => "Y",
                        }
                    }
                }
            },
            CardType::Colored(card) => {
                ctype = "C";
                face = match card.face {
                    Face::Skip => "S",
                    Face::DrawTwo => "D",
                    Face::Reverse => "R",
                    Face::Zero =>  "0",
                    Face::One =>   "1",
                    Face::Two =>   "2",
                    Face::Three => "3",
                    Face::Four =>  "4",
                    Face::Five =>  "5",
                    Face::Six =>   "6",
                    Face::Seven => "7",
                    Face::Eight => "8",
                    Face::Nine =>  "9",
                };

                color = match card.color {
                    Color::Red => "R",
                    Color::Green => "G",
                    Color::Blue => "B",
                    Color::Yellow => "Y",
                }
            },
        }

        write!(f, "{};{};{}", ctype, face, color)
    }
}

impl CardType {
    pub fn display_name(&self) -> String {
        match self {
            CardType::Wild(wild) => {
                match wild.face {
                    WildFace::DrawFour => "Draw Four".into(),
                    WildFace::ColorWild(_) => "Wild Card".into(),
                }
            },
            CardType::Colored(card) => {
                match card.face {
                    Face::DrawTwo => format!("{:?} Draw Two", card.color),
                    Face::Skip => format!("{:?} Skip", card.color),
                    Face::Reverse => format!("{:?} Reverse", card.color),
                    face => format!("{:?} {:?}", card.color, face),
                }
            },
        }
    }
}

impl FromStr for CardType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<CardType, Self::Err> {
        let indexes: Vec<_> = s.split_terminator(';').collect();
        if indexes.len() < 3 {
            return Err("Cards take the form of \"<TYPE>;<FACE>;[COLOR]\", e.g. W;D;_ for a draw four.");
        }

        match indexes[0] {
            val if !&["w", "W", "C", "c"].contains(&val) => return Err("Invalid card type. Must either be W or C."),
            "w" | "W" => {
                let face = match indexes[1] {
                    "D" => WildFace::DrawFour,
                    "C" => {
                        let color = indexes[2].parse()?;
                        WildFace::ColorWild(color)
                    },
                    _ => return Err("Invalid face for wild card. Must be either D or C."),
                };

                Ok(WildCard::new(face).into())
            },
            "c" | "C" => {
                let color = indexes[2].parse()?;
                let face = match indexes[1] {
                    "s" | "S" => Face::Skip,
                    "d" | "D" => Face::DrawTwo,
                    "r" | "R" => Face::Reverse,
                    num => {
                        let digit = match num.parse::<isize>() {
                            Ok(d) => d,
                            Err(_) => return Err("Invalid face for color card. Must be S, D, R, or a number from 0 to 9 inclusive."),
                        };

                        digit.into()
                    }
                };

                Ok(ColorCard::new(color, face).into())
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        if s.is_empty() {
            return Err("Color cannot be empty.");
        }

        match s.chars().nth(0) {
            None => Err("Cannot get first character of color."),
            Some(ch) => match ch {
                'y' | 'Y' => Ok(Color::Yellow),
                'r' | 'R' => Ok(Color::Red),
                'b' | 'B' => Ok(Color::Blue),
                'g' | 'G' => Ok(Color::Green),
                _ => Err("Colors must be one of Y, R, G, or B."),
            },
        }
    }
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

    #[test]
    fn test_card_fromstr() {
        let yellow_zero: CardType = "C;0;Y".parse().unwrap();
        assert_eq!(CardType::Colored(ColorCard::new(Color::Yellow, Face::Zero)), yellow_zero);

        let red_wild: CardType = "W;C;R".parse().unwrap();
        assert_eq!(CardType::Wild(WildCard::new(WildFace::ColorWild(Color::Red))), red_wild);
    }

    #[test]
    #[should_panic]
    fn test_invalid_cardtype_fromstr() {
        let invalid: CardType = "Dummy; ;Tuna".parse().unwrap(); //panic
    }

    #[test]
    #[should_panic]
    fn test_invalid_cardface_fromstr() {
        let invalids = (
            "W;Invalid face;_".parse::<CardType>(), 
            "C;Invalid face;R".parse::<CardType>()
        );

        match invalids {
            (Ok(a), _) | (_, Ok(a)) => return, //failed test
            (Err(a), Err(b)) => panic!("Working as expected."),
        }
    }

    #[test]
    fn test_color_fromstr() {
        let color = "Y".parse().unwrap();
        assert_eq!(Color::Yellow, color);

        let color = "R".parse().unwrap();
        assert_eq!(Color::Red, color);

        let color = "G".parse().unwrap();
        assert_eq!(Color::Green, color);

        let color = "B".parse().unwrap();
        assert_eq!(Color::Blue, color);
    }

    #[test]
    #[should_panic]
    fn invalid_color_fromstr() {
        let color: Color = "Z".parse().unwrap();
    }
}