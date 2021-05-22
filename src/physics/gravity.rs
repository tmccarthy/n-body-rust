use crate::physics::primitives::{Force, Scalar, Vector2D};
use crate::physics::Body;

struct GravitationalConstant(Scalar);

struct Gravity {
    gravitational_constant: GravitationalConstant,
}

impl Gravity {
    pub fn due_to(self: Gravity, object: Body, subject: Body) -> Force {
        let position_difference: Vector2D = subject.position.0 - object.position.0;
        let distance = position_difference.magnitude();

        let force_magnitude =
            (self.gravitational_constant.0 * object.mass.0 * subject.mass.0) / distance.powi(2);
        let force_direction = position_difference.unit();

        Force(force_direction * force_magnitude)
    }
}
