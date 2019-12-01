use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
            Direction::South => write!(f, "South"),
        }
    }
}
