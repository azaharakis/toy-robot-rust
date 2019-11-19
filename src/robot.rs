use super::point;

#[derive(Debug)]
pub struct Robot<'a> {
    place: &'a point::Point
}

impl<'a> Robot<'a> {
    pub fn new(p : &'a point::Point) -> Robot {
        Robot { place: p }
    }
    pub fn get_position(&self) -> &'a point::Point {
        self.place
    }
}