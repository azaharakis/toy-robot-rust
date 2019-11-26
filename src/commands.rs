use crate::point::Point;
use crate::{point, Direction};
use std::fmt::Error;

pub enum MoveCommands {
    Move,
    Left,
    Right,
}

pub enum StartingCommand {
    Place(point::Point, Direction),
}

pub enum KnownCommands {
    MoveCommands(MoveCommands),
    StartingCommand(StartingCommand),
}

struct StartingCommands {
    place: fn(Point, Direction),
}

struct Commands {
    left: fn(),
    right: fn(),
    perform_move: fn(),
    report: (),
}

pub fn get_commands<F>(
    validate_starting_command: F,
) -> Result<(StartingCommand, Vec<MoveCommands>), Error>
where
    F: Fn(&Point, &Direction) -> bool,
{
    let commands = vec![
        KnownCommands::StartingCommand(StartingCommand::Place(
            point::Point { x: 1, y: 1 },
            Direction::North,
        )),
        KnownCommands::MoveCommands(MoveCommands::Move),
    ];

    let mut place_command: Option<StartingCommand> = None;
    let move_commands = commands
        .into_iter()
        .filter_map(|c| match c {
            KnownCommands::StartingCommand(s) => {
                match s {
                    StartingCommand::Place(p, d) => {
                        if validate_starting_command(&p, &d) {
                            place_command = Some(StartingCommand::Place(p, d))
                        } else {
                            println!("Cannot place robot at: {:?}", p);
                        }
                    }
                }
                None
            }
            KnownCommands::MoveCommands(m) => {
                if let Some(_) = place_command {
                    return Some(m);
                }
                None
            }
        })
        .collect();

    Ok((place_command.unwrap(), move_commands))
}
