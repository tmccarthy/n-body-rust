use crate::physics::primitives::*;

mod primitives;

struct Body {
    mass: Mass,
    position: Position,
    velocity: Velocity,
}
