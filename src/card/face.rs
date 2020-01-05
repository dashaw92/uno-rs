#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Face {
    DrawFour,
    ColorCard,
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

impl Face {
    pub fn from(ch: char) -> Option<Face> {
        match ch {
            '0' => Some(Face::Zero),
            '1' => Some(Face::One),
            '2' => Some(Face::Two),
            '3' => Some(Face::Three),
            '4' => Some(Face::Four),
            '5' => Some(Face::Five),
            '6' => Some(Face::Six),
            '7' => Some(Face::Seven),
            '8' => Some(Face::Eight),
            '9' => Some(Face::Nine),
            'S' => Some(Face::Skip),
            'R' => Some(Face::Reverse),
            'D' => Some(Face::DrawFour),
            'T' => Some(Face::DrawTwo),
            'C' => Some(Face::ColorCard),
            _ => None,
        }
    }
}

impl Into<char> for Face {
    fn into(self) -> char {
        match self {
            Face::DrawFour => 'D',
            Face::ColorCard => 'C',
            Face::Reverse => 'R',
            Face::Skip => 'S',
            Face::DrawTwo => 'T',
            Face::Zero => '0',
            Face::One => '1',
            Face::Two => '2',
            Face::Three => '3',
            Face::Four => '4',
            Face::Five => '5',
            Face::Six => '6',
            Face::Seven => '7',
            Face::Eight => '8',
            Face::Nine => '9',
        }
    }
}
