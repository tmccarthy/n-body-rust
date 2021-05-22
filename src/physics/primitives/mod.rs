use core::ops;

pub use position::*;
pub use vector::*;
use std::ops::{Mul, Add};

mod position;
mod vector;

pub type Scalar = f64;

// Scalars
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TemporalDuration(pub Scalar);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mass(pub Scalar);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Velocity(pub Vector2D);

impl ops::Mul<TemporalDuration> for Velocity {
    type Output = Position;

    fn mul(self, rhs: TemporalDuration) -> Self::Output {
        Position(self.0 * rhs.0)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Acceleration(pub Vector2D);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Force(pub Vector2D);

impl ops::Add<Force> for Force {
    type Output = Force;

    fn add(self, rhs: Force) -> Self::Output {
        Force(self.0 + rhs.0)
    }
}

impl ops::Mul<Mass> for Force {
    type Output = Acceleration;

    fn mul(self, rhs: Mass) -> Self::Output {
        Acceleration(self.0 * rhs.0)
    }
}

impl ops::Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, rhs: Mass) -> Self::Output {
        Acceleration(self.0 / rhs.0)
    }
}
