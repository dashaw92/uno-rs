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

impl<S> From<S> for Face 
where S: AsRef<str> 
{
    fn from(input: S) -> Face {
        match input.as_ref() {
            "0" => Face::Zero,
            "1" => Face::One,
            "2" => Face::Two,
            "3" => Face::Three,
            "4" => Face::Four,
            "5" => Face::Five,
            "6" => Face::Six,
            "7" => Face::Seven,
            "8" => Face::Eight,
            "9" => Face::Nine,
            "S" => Face::Skip,
            "R" => Face::Reverse,
            "D" => Face::DrawFour,
            "T" => Face::DrawTwo,
            "C" => Face::ColorCard,
            _ => unreachable!(),
        }
    }
}

impl Into<&'static str> for Face {
    fn into(self) -> &'static str {
        match self {
            Face::DrawFour => "D",
            Face::ColorCard => "C",
            Face::Reverse => "R",
            Face::Skip => "S",
            Face::DrawTwo => "T",
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
        }
    }
}