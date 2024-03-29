use crate::point::Point;

#[derive(Fail, Debug)]
pub enum ApplicationErrors {
    #[fail(display = "Could not move to position: {:?}", _0)]
    InvalidMove(Point),
    #[fail(display = "Robot has not been placed yet")]
    RobotHasNotBeenPlacedYet,
}
