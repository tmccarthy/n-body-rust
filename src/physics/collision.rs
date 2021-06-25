use crate::engine::Body;
use crate::physics::primitives::{TemporalDuration, Scalar, Position, Velocity};

pub fn collide(left: &Body, right: &Body, dt: TemporalDuration) -> Option<Body> {
    if (left.position - right.position).0.magnitude() <= collision_radius(left, right) {
        let new_mass = left.mass + right.mass;
        let new_position = Position((left.mass.0 * left.position.0 + right.mass.0 * right.position.0) / new_mass.0);
        let new_velocity = Velocity((left.mass.0 * left.momentum().0 + right.mass.0 * right.momentum().0) / new_mass.0);

        let new_body: Body = Body {
            mass: new_mass,
            position: new_position,
            velocity: new_velocity,
        };

        Some(new_body)
    } else {
        None
    }
}

fn collision_radius(left: &Body, right: &Body) -> Scalar {
    (left.mass.0 + right.mass.0) * 10.0
}