use super::point;
use crate::direction::Direction;
use std::fmt;

pub struct Robot {
    place: point::Point,
    direction: Direction,
}

impl Robot {
    pub fn new(p: point::Point, d: Direction) -> Robot {
        Robot {
            place: p,
            direction: d,
        }
    }

    pub fn get_position(&self) -> &point::Point {
        &self.place
    }

    pub fn get_facing_direction(&self) -> &Direction {
        &self.direction
    }

    // Find out what difference is between mut self | &mut self
    pub fn set_position(&mut self, p: point::Point) {
        self.place = p;
    }

    pub fn set_facing_direction(&mut self, d: Direction) {
        self.direction = d;
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "position: {}, direction: {}", self.place, self.direction)
    }
}
