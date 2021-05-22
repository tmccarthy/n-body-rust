use crate::physics::primitives::*;
use crate::physics::gravity::Gravity;
use std::collections::HashMap;
use crate::physics::numerical_methods::euler_method;

#[derive(Eq, PartialEq, Copy, Clone)]
struct BodyId(u64);

#[derive(Copy, Clone)]
pub struct Body {
    id: BodyId,
    mass: Mass,
    position: Position,
    velocity: Velocity,
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Body {}

pub struct Universe {
    gravity: Gravity,
    bodies: Vec<Body>,
}

impl Gravity {
    fn due_to_bodies(self: &Gravity, object: &Body, subject: &Body) -> Force {
        self.due_to(object.position, object.mass, subject.position, subject.mass)
    }
}

impl Universe {
    pub fn step_forward(self: Universe, dt: TemporalDuration) -> Universe {
        let new_bodies = self.bodies.iter().map(|object: &Body| {
            let new_position = Position(object.position.0 + (object.velocity.0 * dt.0));

            let mut total_force = Force(Vector2D::zero());

            for subject in &self.bodies {
                if *subject != *object {
                    let force: &Force = &self.gravity.due_to_bodies(object, &subject);

                    total_force = Force(total_force.0 + force.0);
                }

            }

            let acceleration = Acceleration(total_force.0 * (1.0 / object.mass.0));

            // TODO this should probably be injected
            let new_velocity = euler_method::next_velocity(acceleration, object.velocity, dt);

            Body {
                position: new_position,
                velocity: new_velocity,
                ..(*object)
            }
        }).collect();

        Universe {
            bodies: new_bodies,
            ..(self)
        }
    }
}
