mod board;
mod robot;

enum KnownCommands {
    Place((i32, i32)),
//    North,
//    East,
//    South,
//    West,
//    Move,
}

fn main() {
    let commands: Vec<KnownCommands> = vec![KnownCommands::Place((1, 1))];
    let _board = board::Board::new(5, 5);
    let mut _robot: robot::Robot;

    for command in commands.iter() {
        match command {
            KnownCommands::Place((x, y)) => {
                if _board.is_valid_position((x, y)) {
                    _robot = robot::Robot::new(x.clone(), y.clone());
                    return println!("Successfuly placed robot at {:?}", _robot.get_position());
                }
                return println!("Cannot place robot at: {}, {}", x, y);
            }
        }
    }
//
}
