use core::ops;
use std::ops::Add;

pub use position::*;
pub use vector::*;

mod position;
mod vector;

pub type Scalar = f64;

// Scalars
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TemporalDuration(pub Scalar);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mass(pub Scalar);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Distance(pub Scalar);

impl Distance {
    pub fn between(left: Position, right: Position) -> Distance {
        Distance((left - right).0.magnitude())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Velocity(pub Vector2D);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Acceleration(pub Vector2D);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Force(pub Vector2D);
