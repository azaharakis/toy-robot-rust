use core::borrow::Borrow;

pub struct Board {
    size: (i32, i32)
}

impl Board {
    pub fn new(x: i32, y: i32) -> Board {
        Board { size: (x, y) }
    }

    pub fn is_valid_position(&self, (x, y): (&i32, &i32)) -> bool {
        if x >= 0.borrow() && x <= self.size.0.borrow() && y >= 0.borrow() && y <= self.size.1.borrow() { true } else { false }
    }
}