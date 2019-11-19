use core::borrow::Borrow;

mod board;
mod robot;
mod point;

enum KnownCommands {
    Place(point::Point),
    North,
}

fn main() {
    let _board = board::Board::new(5, 5);
    let mut _robot: Option<robot::Robot> = None;

    let commands: Vec<KnownCommands> = vec![KnownCommands::Place(point::Point { x: 1, y: 1 }), KnownCommands::North];

    for command in commands.iter() {
        match command {
            KnownCommands::Place(p) => {
                if _board.is_valid_position(&p) {
                    _robot = Some(robot::Robot::new(p));
                    println!("Successfuly placed robot {:?}", p);
                } else {
                    println!("Cannot place robot at: {:?}", p);
                }

            }
            KnownCommands::North => {

            }
        }
    }
//
}
