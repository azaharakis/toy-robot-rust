use crate::direction::Direction;
use crate::errors::ApplicationErrors;
use crate::point;
use crate::point::Point;
use crate::robot::Robot;

#[derive(Debug, PartialEq, Eq)]
pub enum KnownCommands {
    Place(point::Point, Direction),
    Move,
    Left,
    Right,
    Report,
}

pub trait Commands {
    fn place(&self, p: Point, d: Direction) -> Option<Robot>;
    fn left(&self, r: &mut Robot);
    fn right(&self, r: &mut Robot);
    fn perform_move(&self, r: &mut Robot) -> Result<(), ApplicationErrors>;
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
        KnownCommands::Move => {
            robot
                .as_mut()
                .map(|r| app.perform_move(r).unwrap_or_else(|e| println!("{}", e)));
        }
        KnownCommands::Left => {
            robot.as_mut().map(|r| app.left(r));
        }
        KnownCommands::Right => {
            robot.as_mut().map(|r| app.right(r));
        }
        KnownCommands::Report => {
            robot.as_mut().map(|r| println!("Robot placed at: {}", r));
        }
    });
}
