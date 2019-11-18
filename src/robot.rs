pub struct Robot {
    place: (i32, i32)
}

impl Robot {
    pub fn new(x: i32, y: i32) -> Robot {
        Robot { place: (x, y) }
    }
    pub fn get_position(&self) -> (&i32, &i32) {
        (&self.place.0, &self.place.1)
    }
}