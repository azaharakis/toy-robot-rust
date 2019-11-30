use crate::direction::Direction;
use crate::robot::Robot;
use crate::{point, Commands};

pub enum KnownCommands {
    Place(point::Point, Direction),
    Move,
    Left,
    Right,
    Report,
}

pub fn run_commands_against(app: impl Commands) {
    let commands = vec![
        KnownCommands::Place(point::Point { x: 5, y: 5 }, Direction::North),
        KnownCommands::Left,
        KnownCommands::Right,
        KnownCommands::Right,
        KnownCommands::Move,
        KnownCommands::Report,
    ];
    let mut robot: Option<Robot> = None;

    commands.into_iter().for_each(|command| match command {
        KnownCommands::Place(p, d) => {
            robot = app.place(p, d);
        }
        c => match robot.as_mut() {
            Some(r) => match c {
                KnownCommands::Move => app.perform_move(r).unwrap_or_else(|e| println!("{}", e)),
                KnownCommands::Left => app.left(r),
                KnownCommands::Right => app.right(r),
                KnownCommands::Report => println!("Robot placed at: {}", r),
                _ => {}
            },
            _ => {}
        },
    });
}
