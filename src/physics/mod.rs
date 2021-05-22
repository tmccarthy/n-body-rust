use crate::physics::primitives::*;

mod gravity;
mod numerical_methods;
mod primitives;

struct Body {
    mass: Mass,
    position: Position,
    velocity: Velocity,
}
