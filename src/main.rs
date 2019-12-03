#[macro_use]
extern crate failure;
use crate::board::Board;
use crate::commands::{run_commands_against, Commands};
use crate::direction::Direction;
use crate::errors::ApplicationErrors;
use crate::robot::Robot;

mod board;
mod commands;
mod direction;
mod errors;
mod movement;
mod parse_input;
mod point;
mod robot;

struct App {
    board: Board,
    robot: Option<Robot>,
}

impl App {
    fn new() -> App {
        App {
            board: Board::new(5, 5),
            robot: None,
        }
    }
}

impl Commands for App {
    fn place(&mut self, p: point::Point, d: Direction) {
        if self.board.is_valid_position(&p) {
            self.robot = Some(Robot::new(p, d));
        }
    }
    fn left(&mut self) {
        self.robot.as_mut().map(|r| {
            r.set_facing_direction(movement::get_direction_to_left(r.get_facing_direction()))
        });
    }
    fn right(&mut self) {
        self.robot.as_mut().map(|r| {
            r.set_facing_direction(movement::get_direction_to_right(r.get_facing_direction()))
        });
    }
    fn perform_move(&mut self) -> Result<(), ApplicationErrors> {
        // assigning to a local variable as compiler will cause issues if &self is referenced
        // in the closure, it cannot tell if self.robot would be mutated as well.
        let board = &self.board;
        self.robot.as_mut().map_or_else(
            || Err(ApplicationErrors::RobotHasNotBeenPlacedYet),
            |r| {
                let potential_pos =
                    movement::move_a_point_in_direction(r.get_facing_direction(), r.get_position());
                if board.is_valid_position(&potential_pos) {
                    Ok(r.set_position(potential_pos))
                } else {
                    Err(ApplicationErrors::InvalidMove(potential_pos))
                }
            },
        )
    }
    fn report(&self) {
        self.robot
            .as_ref()
            .map(|r| println!("Robot placed at {}", r));
    }
}

fn main() {
    run_commands_against(App::new())
}
