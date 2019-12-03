use crate::direction::Direction;
use crate::errors::ApplicationErrors;
use crate::point;
use crate::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub enum KnownCommands {
    Place(point::Point, Direction),
    Move,
    Left,
    Right,
    Report,
}

pub trait Commands {
    fn place(&mut self, p: Point, d: Direction);
    fn left(&mut self);
    fn right(&mut self);
    fn perform_move(&mut self) -> Result<(), ApplicationErrors>;
    fn report(&self);
}

pub fn run_commands_against(mut app: impl Commands) {
    let commands = vec![
        KnownCommands::Place(point::Point { x: 5, y: 5 }, Direction::North),
        KnownCommands::Left,
        KnownCommands::Right,
        KnownCommands::Right,
        KnownCommands::Move,
        KnownCommands::Report,
    ];

    commands.into_iter().for_each(|command| match command {
        KnownCommands::Place(p, d) => {
            app.place(p, d);
        }
        KnownCommands::Move => app.perform_move().unwrap_or_else(|e| println!("{}", e)),
        KnownCommands::Left => app.left(),
        KnownCommands::Right => {
            app.right();
        }
        KnownCommands::Report => {
            app.report();
        }
    });
}
