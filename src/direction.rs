#[derive(PartialEq)]
pub enum Direction{
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    pub fn opposite(&self, other: &Direction) -> bool {
        let opposite = match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down=> Self::Up,
            Self::Right => Self::Left, 
        };

        return opposite == *other;
    }
}