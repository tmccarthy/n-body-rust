use core::ops;
use std::ops::Add;

mod vector;
mod position;

pub use vector::*;
pub use position::*;

// Scalars
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mass(pub f64);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Distance(pub f64);

impl Distance {
    pub fn between(left: Position, right: Position) -> Distance {
        Distance((left - right).0.magnitude())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Velocity(Vector2D);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Acceleration(Vector2D);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Force(Vector2D);
