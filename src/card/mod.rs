pub mod color;
pub mod face;

use face::Face;
use color::*;

use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Card {
    pub color: Color,
    pub face: Face,
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color: char = self.color.into();
        let face: char = self.face.into();

        write!(f, "{}{}", color, face)
    }
}

impl Card {
    pub fn new(color: Color, face: Face) -> Self {
        Card { color, face }
    }

    pub fn display_name(&self) -> String {
        match self.face {
            Face::DrawFour => "Draw Four".into(),
            Face::ColorCard => "Wild Card".into(),
            Face::DrawTwo => format!("{:?} Draw Two", self.color),
            Face::Skip => format!("{:?} Skip", self.color),
            Face::Reverse => format!("{:?} Reverse", self.color),
            _ => format!("{:?} {:?}", self.color, self.face),
        }
    }

    pub fn can_play_on<C: Into<Card>>(&self, other: C) -> bool {
        let card = other.into();
        match self.face {
            Face::ColorCard | Face::DrawFour => true,
            _ => self.color == card.color || self.face == card.face,
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Card, Self::Err> {
        let color = match s.chars().nth(0)
                        .map(|x| x.to_string())
                        .and_then(|col| col.parse::<Color>().ok()) {
            Some(color) => color,
            None => return Err("Could not parse color."),
        };
        let face = match s.chars().nth(1).map(Face::from).flatten() {
            Some(face) => face,
            None => return Err("Invalid face identifier."),
        };

        Ok(Card::new(color, face))
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_rules() {
        let draw_four = Card::new(Color::Yellow, Face::DrawFour);
        let draw_four2 = Card::new(Color::Red, Face::DrawFour);
        assert!(draw_four.can_play_on(draw_four2));
        
        let green_draw_two = Card::new(Color::Green, Face::DrawTwo);
        let red_draw_two = Card::new(Color::Red, Face::DrawTwo);
        assert!(green_draw_two.can_play_on(red_draw_two));
        assert!(red_draw_two.can_play_on(green_draw_two));

        let yellow_zero = Card::new(Color::Yellow, Face::Zero);
        let blue_nine = Card::new(Color::Blue, Face::Nine);
        assert!(!yellow_zero.can_play_on(blue_nine));
        assert!(!blue_nine.can_play_on(yellow_zero));

        assert!(yellow_zero.can_play_on(draw_four));
        assert!(draw_four2.can_play_on(yellow_zero))
    }

    #[test]
    fn test_card_fromstr() {
        let yellow_zero: Card = "Y0".parse().unwrap();
        assert_eq!(Card::new(Color::Yellow, Face::Zero), yellow_zero);

        let red_wild: Card = "RC".parse().unwrap();
        assert_eq!(Card::new(Color::Red, Face::ColorCard), red_wild);
    }

    #[test]
    #[should_panic]
    fn test_invalid_card_fromstr() {
        let invalid: Card = "Invalid Card".parse().unwrap(); //panic
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