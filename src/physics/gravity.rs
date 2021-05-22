use crate::physics::primitives::{Force, Mass, Position, Scalar, Vector2D};

#[derive(Copy, Clone)]
pub struct GravitationalConstant(Scalar);

#[derive(Copy, Clone)]
pub struct Gravity {
    gravitational_constant: GravitationalConstant,
}

impl Gravity {
    pub fn due_to(
        self: Gravity,
        object_position: Position,
        object_mass: Mass,
        subject_position: Position,
        subject_mass: Mass,
    ) -> Force {
        let position_difference: Vector2D = subject_position.0 - object_position.0;
        let distance = position_difference.magnitude();

        let force_magnitude =
            (self.gravitational_constant.0 * object_mass.0 * subject_mass.0) / distance.powi(2);
        let force_direction = position_difference.unit();

        Force(force_direction * force_magnitude)
    }
}
