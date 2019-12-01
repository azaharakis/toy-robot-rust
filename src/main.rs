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
}

impl App {
    fn new() -> App {
        App {
            board: Board::new(5, 5),
        }
    }
}

impl Commands for App {
    fn place(&self, p: point::Point, d: Direction) -> Option<Robot> {
        if self.board.is_valid_position(&p) {
            return Some(Robot::new(p, d));
        }
        return None;
    }
    fn left(&self, robot: &mut Robot) {
        robot.set_facing_direction(movement::get_direction_to_left(
            robot.get_facing_direction(),
        ));
    }
    fn right(&self, robot: &mut Robot) {
        robot.set_facing_direction(movement::get_direction_to_right(
            robot.get_facing_direction(),
        ));
    }
    fn perform_move(&self, robot: &mut Robot) -> Result<(), ApplicationErrors> {
        let potential_pos =
            movement::move_a_point_in_direction(robot.get_facing_direction(), robot.get_position());
        if self.board.is_valid_position(&potential_pos) {
            Ok(robot.set_position(potential_pos))
        } else {
            Err(ApplicationErrors::InvalidMove(potential_pos))
        }
    }
}

fn main() {
    run_commands_against(App::new())
}
