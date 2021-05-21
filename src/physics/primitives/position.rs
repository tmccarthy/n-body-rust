use core::ops;

use crate::physics::primitives::vector::Vector2D;

// Vector quantities
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position(pub Vector2D);

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0)
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0)
    }
}
