use crate::physics::primitives::*;

mod primitives;
mod numerical_methods;

struct Body {
    mass: Mass,
    position: Position,
    velocity: Velocity,
}
