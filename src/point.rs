use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    //Why do I need to derive from copy, clone? to add values together?
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}
