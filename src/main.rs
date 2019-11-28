use crate::board::Board;
use crate::commands::{run_commands_against, Commands};
use crate::robot::{Movement, Robot};

mod board;
mod commands;
mod point;
mod robot;

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

fn app() -> Commands<Robot> {
    Commands {
        place: Box::new(|p, d| {
            let board = Board::new(5, 5);
            if board.is_valid_position(&p) {
                return Some(Robot::new(p, d));
            }
            return None;
        }),
        perform_move: Box::new(|robot| {
            let board = Board::new(5, 5);
            let potential_pos = Movement::move_a_point_in_direction(
                robot.get_facing_direction(),
                robot.get_position(),
            );
            if board.is_valid_position(&potential_pos) {
                robot.set_position(potential_pos);
                println!("Moved the robot to {:?}", robot.get_position())
            } else {
                println!("Moving to {:?} would be invalid", &potential_pos)
            }
        }),
        left: Box::new(|robot| {
            robot.set_facing_direction(Movement::get_direction_to_left(
                robot.get_facing_direction(),
            ));
            println!("Moved the robot to {:?}", robot.get_facing_direction())
        }),
        right: Box::new(|robot| {
            robot.set_facing_direction(Movement::get_direction_to_right(
                robot.get_facing_direction(),
            ));
            println!("Moved the robot to {:?}", robot.get_facing_direction())
        }),
    }
}

fn main() {
    run_commands_against(app)
}
