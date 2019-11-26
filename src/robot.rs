use super::point;
use super::Direction;

pub struct Robot {
    place: point::Point,
    direction: Direction,
}
// TODO: these impls should be part of the command traits?

impl Robot {
    pub fn new(p: point::Point, d: Direction) -> Robot {
        Robot { place: p, direction: d}
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

pub trait Command {
    fn report(&self);
}

impl Command for Robot {
    fn report(&self){

    }
}

pub struct Movement;

impl Movement {
    pub fn get_direction_to_left(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn get_direction_to_right(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    pub fn move_a_point_in_direction(d: &Direction, p: &point::Point) -> point::Point {
        match d {
            Direction::North => {
                p.clone() + point::Point { x: 0, y: 1 }
            }
            Direction::East => {
                p.clone() + point::Point { x: -1, y: 0 }
            }
            Direction::South => {
                p.clone() + point::Point { x: 0, y: -1 }
            }
            Direction::West => {
                p.clone() + point::Point { x: 1, y: 0 }
            }
        }
    }
}