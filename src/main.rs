mod board;
mod robot;
mod point;

use robot::Movement;

pub enum KnownCommands {
    Place(point::Point, Direction),
    Move,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let _board = board::Board::new(5, 5);
    let mut _robot: Option<robot::Robot> = None;

    let commands: Vec<KnownCommands> = vec![KnownCommands::Place(point::Point { x: 1, y: 1 }, Direction::North),
                                            KnownCommands::Left, KnownCommands::Left, KnownCommands::Right,
                                            KnownCommands::Move, KnownCommands::Move];

    for command in commands {
        match command {
            KnownCommands::Place(p, d) => {
                if _board.is_valid_position(&p) {
                    _robot = Some(robot::Robot::new(p, d));
                    println!("Susccessfuly placed robot {:?}", _robot.as_ref().unwrap().get_position());
                } else {
                    println!("Cannot place robot at: {:?}", p);
                }
            }
            KnownCommands::Move => {
                match _robot.as_mut() {
                    Some(r) => {
                        let potential_pos = Movement::move_a_point_in_direction(r.get_facing_direction(), r.get_position());
                        if _board.is_valid_position(&potential_pos) {
                            r.set_position(potential_pos);
                            println!("Moved the robot to {:?}", r.get_position())
                        } else {
                            println!("Moving to {:?} would be invalid", &potential_pos)
                        }

                    }
                    None => println!("Cannot move a robot that has not been placed")
                }
            }
            KnownCommands::Left => {
                match _robot.as_mut() {
                    Some(r) => {
                        r.set_facing_direction(Movement::get_direction_to_left(r.get_facing_direction()));
                        println!("Moved the robot to {:?}", r.get_facing_direction())
                    }
                    None => println!("Cannot turn a robot that has not been placed")
                }
            }
            KnownCommands::Right => {
                match _robot.as_mut() {
                    Some(r) => {
                        r.set_facing_direction(Movement::get_direction_to_right(r.get_facing_direction()));
                        println!("Moved the robot to {:?}", r.get_facing_direction())
                    }
                    None => println!("Cannot turn a robot that has not been placed")
                }
            }
        }
    }
//
}
