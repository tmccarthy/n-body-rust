use core::ops;

use crate::physics::primitives::Scalar;
use crate::physics::primitives::{Acceleration, TemporalDuration, Vector2D, Velocity};

pub fn next_velocity(
    acceleration: Acceleration,
    velocity: Velocity,
    dt: TemporalDuration,
) -> Velocity {
    Velocity(next_y(acceleration.0, velocity.0, dt.0))
}

pub fn next_y(y_prime: Vector2D, y_0: Vector2D, h: Scalar) -> Vector2D {
    y_0 + (y_prime * h)
}
