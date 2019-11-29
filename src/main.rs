use crate::board::Board;
use crate::commands::{run_commands_against, Commands};
use crate::direction::Direction;
use crate::movement::Movement;
use crate::robot::Robot;

mod board;
mod commands;
mod direction;
mod movement;
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

impl Commands<Robot> for App {
    fn place(&self, p: point::Point, d: Direction) -> Option<Robot> {
        if self.board.is_valid_position(&p) {
            return Some(Robot::new(p, d));
        }
        return None;
    }
    fn left(&self, robot: &mut Robot) {
        robot.set_facing_direction(Movement::get_direction_to_left(
            robot.get_facing_direction(),
        ));
        println!("Moved the robot to {:?}", robot.get_facing_direction())
    }
    fn right(&self, robot: &mut Robot) {
        robot.set_facing_direction(Movement::get_direction_to_right(
            robot.get_facing_direction(),
        ));
        println!("Moved the robot to {:?}", robot.get_facing_direction())
    }
    fn perform_move(&self, robot: &mut Robot) {
        let potential_pos =
            Movement::move_a_point_in_direction(robot.get_facing_direction(), robot.get_position());
        if self.board.is_valid_position(&potential_pos) {
            robot.set_position(potential_pos);
            println!("Moved the robot to {:?}", robot.get_position())
        } else {
            println!("Moving to {:?} would be invalid", &potential_pos)
        }
    }
}

fn main() {
    run_commands_against(App::new())
}
