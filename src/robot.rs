use super::point;
use super::Direction;

pub struct Robot<> {
    place: point::Point,
    direction: Direction,
}

impl Robot {
    pub fn new(p: point::Point, d: Direction) -> Robot {
        Robot { place: p, direction: d }
    }

    pub fn get_facing_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_position(&self) -> &point::Point {
        &self.place
    }
}


trait Movement {
    fn move_(&mut self) {}
    fn left(&mut self) {}
    fn right(&mut self) {}
}

impl Movement for Robot {
    fn move_(&mut self) {
        match &self.direction {
            Direction::North => {
                self.place = self.place + point::Point { x: 0, y: 1 }
            }
        }
    }
}