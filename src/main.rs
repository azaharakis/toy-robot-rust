mod board;
mod robot;
mod point;

enum KnownCommands {
    Place(point::Point, Direction),
    Move,
}

#[derive(Debug)]
pub enum Direction {
    North
}

fn main() {
    let _board = board::Board::new(5, 5);
    let mut _robot: Option<robot::Robot> = None;

    let commands: Vec<KnownCommands> = vec![KnownCommands::Place(point::Point { x: 1, y: 1 }, Direction::North), KnownCommands::Move];

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
                match _robot.as_ref() {
                    Some(r) => {
                        match r.get_facing_direction() {
                            Direction::North => {
                                if _board.is_valid_position(r.get_position()) {
                                    println!("Robot facing North");
                                }
                            }
                        }
                    }
                    None => println!("Cannot move a robot that has not been placed")
                }
            }
        }
    }
//
}
