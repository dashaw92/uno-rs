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
        s.chars()
            .nth(0)
            .and_then(|ch| match ch {
                'y' | 'Y' => Some(Color::Yellow),
                'r' | 'R' => Some(Color::Red),
                'b' | 'B' => Some(Color::Blue),
                'g' | 'G' => Some(Color::Green),
                _ => None,
            })
            .ok_or("Colors must be one of Y, R, G, or B.")
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
