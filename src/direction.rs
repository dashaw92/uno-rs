use std::ops::{AddAssign, Not};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum GameDirection {
    Clockwise,
    CounterClockwise,
}

impl GameDirection {
    pub fn reverse(self) -> GameDirection {
        match self {
            GameDirection::Clockwise => GameDirection::CounterClockwise,
            GameDirection::CounterClockwise => GameDirection::Clockwise,
        }
    }
}

impl AddAssign<GameDirection> for usize {
    fn add_assign(&mut self, rhs: GameDirection) {
        match rhs {
            GameDirection::Clockwise => *self += 1,
            GameDirection::CounterClockwise => *self -= 1,
        }
    }
}

impl Not for GameDirection {
    type Output = GameDirection;

    fn not(self) -> Self::Output {
        self.reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_tests() {
        let direction = GameDirection::Clockwise;
        assert_eq!(direction, !!direction);
    }
}