use core::ops;

pub use position::*;
use std::ops::{Add, Mul};
pub use vector::*;

mod position;
mod vector;

pub type Scalar = f64;

// Scalars
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TemporalDuration(pub Scalar);

impl Mul<Scalar> for TemporalDuration {
    type Output = TemporalDuration;

    fn mul(self, rhs: f64) -> Self::Output {
        TemporalDuration(self.0 * rhs)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mass(pub Scalar);

impl Add<Mass> for Mass {
    type Output = Mass;

    fn add(self, rhs: Mass) -> Self::Output {
        Mass(self.0 + rhs.0)
    }
}

impl Mul<Velocity> for Mass {
    type Output = Momentum;

    fn mul(self, rhs: Velocity) -> Self::Output {
        Momentum(self.0 * rhs.0)
    }
}

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

pub struct Momentum(pub Vector2D);

impl ops::Add<Momentum> for Momentum {
    type Output = Momentum;

    fn add(self, rhs: Momentum) -> Self::Output {
        Momentum(self.0 + rhs.0)
    }
}
