use super::point;

pub struct Board {
    size: (i32, i32)
}

impl Board {
    pub fn new(x: i32, y: i32) -> Board {
        Board { size: (x, y) }
    }

    pub fn is_valid_position(&self, p: &point::Point) -> bool {
        if p.x >= 0 && p.x <= self.size.0 && p.y >= 0 && p.y <= self.size.1 { true } else { false }
    }
}