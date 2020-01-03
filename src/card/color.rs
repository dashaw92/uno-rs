use std::str::FromStr;

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

impl Into<char> for Color {
    fn into(self) -> char {
        match self {
            Color::Red => 'R',
            Color::Green => 'G',
            Color::Blue => 'B',
            Color::Yellow => 'Y',
        }
    }
}