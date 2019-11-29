use crate::direction::Direction;
use crate::point;

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
        Direction::North => p.clone() + point::Point { x: 0, y: 1 },
        Direction::East => p.clone() + point::Point { x: -1, y: 0 },
        Direction::South => p.clone() + point::Point { x: 0, y: -1 },
        Direction::West => p.clone() + point::Point { x: 1, y: 0 },
    }
}
