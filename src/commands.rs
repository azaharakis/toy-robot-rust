use crate::point::Point;
use crate::{point, Direction};

pub enum KnownCommands {
    Place(point::Point, Direction),
    Move,
    Left,
    Right,
}

pub struct Commands<T> {
    pub place: Box<dyn Fn(Point, Direction) -> Option<T>>,
    pub left: Box<dyn FnMut(&mut T)>,
    pub right: Box<dyn FnMut(&mut T)>,
    pub perform_move: Box<dyn FnMut(&mut T)>,
}

pub fn run_commands_against<T>(_app: fn() -> Commands<T>) {
    let commands = vec![
        KnownCommands::Place(point::Point { x: 1, y: 1 }, Direction::North),
        KnownCommands::Left,
        KnownCommands::Right,
        KnownCommands::Right,
        KnownCommands::Move,
    ];
    let mut app = _app();
    let mut object: Option<T> = None;

    commands
        .into_iter()
        .map(|command| match command {
            KnownCommands::Place(p, d) => {
                object = (app.place)(p, d);
            }
            c @ KnownCommands::Move | c @ KnownCommands::Left | c @ KnownCommands::Right => {
                match object.as_mut() {
                    Some(o) => match c {
                        KnownCommands::Move => (app.perform_move)(o),
                        KnownCommands::Left => (app.left)(o),
                        KnownCommands::Right => (app.right)(o),
                        _ => {}
                    },
                    _ => {}
                }
            }
        })
        .collect()
}
