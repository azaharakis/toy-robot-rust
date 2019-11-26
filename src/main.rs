use crate::board::Board;
use crate::commands::{get_commands, MoveCommands, StartingCommand};
use crate::point::Point;
use crate::robot::{Movement, Robot};
use std::fmt::Error;

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

fn main() {
    let board: Board = Board::new(5, 5);
    let mut robot: Robot;
    let (place, commands) =
        get_commands(|p, _| board.is_valid_position(p)).expect("Invalid arguments");

    match place {
        StartingCommand::Place(p, d) => robot = Robot::new(p, d),
    };

    for command in commands {
        match command {
            MoveCommands::Move => {
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
            }

            MoveCommands::Left => {
                robot.set_facing_direction(Movement::get_direction_to_left(
                    robot.get_facing_direction(),
                ));
                println!("Moved the robot to {:?}", robot.get_facing_direction())
            }

            MoveCommands::Right => {
                robot.set_facing_direction(Movement::get_direction_to_right(
                    robot.get_facing_direction(),
                ));
                println!("Moved the robot to {:?}", robot.get_facing_direction())
            }
        }
    }
    //
}
