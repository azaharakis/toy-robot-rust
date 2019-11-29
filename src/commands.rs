use crate::direction::Direction;
use crate::point;
use crate::point::Point;

pub enum KnownCommands {
    Place(point::Point, Direction),
    Move,
    Left,
    Right,
}

pub trait Commands<T> {
    fn place(&self, p: Point, d: Direction) -> Option<T>;
    fn left(&self, o: &mut T);
    fn right(&self, o: &mut T);
    fn perform_move(&self, o: &mut T);
}

pub fn run_commands_against<T>(app: impl Commands<T>) {
    let commands = vec![
        KnownCommands::Place(point::Point { x: 1, y: 1 }, Direction::North),
        KnownCommands::Left,
        KnownCommands::Right,
        KnownCommands::Right,
        KnownCommands::Move,
    ];
    let mut object: Option<T> = None;

    commands
        .into_iter()
        .map(|command| match command {
            KnownCommands::Place(p, d) => {
                object = app.place(p, d);
            }
            c @ KnownCommands::Move | c @ KnownCommands::Left | c @ KnownCommands::Right => {
                match object.as_mut() {
                    Some(o) => match c {
                        KnownCommands::Move => app.perform_move(o),
                        KnownCommands::Left => app.left(o),
                        KnownCommands::Right => app.right(o),
                        _ => {}
                    },
                    _ => {}
                }
            }
        })
        .collect()
}
