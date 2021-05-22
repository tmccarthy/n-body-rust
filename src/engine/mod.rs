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
            let new_position = object.position + object.velocity * dt;

            let mut total_force = Force(Vector2D::zero());

            for subject in &self.bodies {
                if *subject != *object {
                    let force: &Force = &self.gravity.due_to_bodies(object, &subject);

                    total_force = total_force + *force;
                }

            }

            let acceleration = total_force / object.mass;

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

    // TODO compute kinetic energy
    // TODO compute potential energy
    // TODO compute total energy
    // TODO compute momentum
}
