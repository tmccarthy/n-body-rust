use std::collections::{HashMap, HashSet};

use crate::physics::collision::collide;
use crate::physics::gravity::Gravity;
use crate::physics::numerical_methods::euler_method;
use crate::physics::primitives::*;

pub mod metrics;

#[derive(Debug, Copy, Clone)]
pub struct Body {
    pub mass: Mass,
    pub position: Position,
    pub velocity: Velocity,
}

impl Body {
    pub fn momentum(&self) -> Momentum {
        self.mass * self.velocity
    }
}

pub struct Universe {
    pub gravity: Gravity,
    pub bodies: Vec<Body>,
}

impl Gravity {
    fn due_to_bodies(self: &Gravity, object: &Body, subject: &Body) -> Force {
        self.due_to(object.position, object.mass, subject.position, subject.mass)
    }
}

impl Universe {
    pub fn add_body(&mut self, body: Body) -> () {
        self.bodies.push(body);
    }

    pub fn step_forward(&self, dt: TemporalDuration) -> Universe {
        let mut new_bodies: Vec<Body> = Vec::with_capacity(self.bodies.len());
        let mut indexes_of_deleted_bodies: HashSet<usize> = HashSet::new();

        for (object_index, object) in self.bodies.iter().enumerate() {
            if indexes_of_deleted_bodies.contains(&object_index) {
                continue;
            }

            // TODO this is incorrect when there is a collision, which edits the position too
            let new_position = object.position + object.velocity * dt;

            let mut total_force = Force(Vector2D::zero());

            let mut object_after_all_collisions: Body = *object;

            for (subject_index, subject) in self.bodies.iter().enumerate() {
                if std::ptr::eq(subject, object) || indexes_of_deleted_bodies.contains(&subject_index) {
                    continue;
                }

                let collision_result: Option<Body> = collide(&object_after_all_collisions, subject, dt);

                if collision_result.is_none() {
                    total_force = total_force + self.gravity.due_to_bodies(&object_after_all_collisions, &subject);
                } else {
                    indexes_of_deleted_bodies.insert(subject_index);
                }

                object_after_all_collisions = collision_result.unwrap_or(object_after_all_collisions);
            }

            let acceleration = total_force / object_after_all_collisions.mass;

            // TODO this should probably be injected
            let new_velocity = euler_method::next_velocity(acceleration, object_after_all_collisions.velocity, dt);

            let new_body = Body {
                position: new_position,
                velocity: new_velocity,
                ..(object_after_all_collisions)
            };

            new_bodies.push(new_body)
        }

        Universe {
            bodies: new_bodies,
            ..(*self)
        }
    }

    pub fn momentum(self: &Universe) -> Momentum {
        self.bodies
            .iter()
            .fold(Momentum(Vector2D::zero()), |acc, body| {
                acc + body.momentum()
            })
    }

    pub fn centre_of_mass(self: &Universe) -> Position {
        // TODO could be more performant
        let total_mass = self
            .bodies
            .iter()
            .fold(Mass(0.0), |acc, body| acc + body.mass);

        Position(
            self.bodies.iter().fold(Vector2D::zero(), |acc, body| {
                acc + (body.mass.0 * body.position.0)
            }) / total_mass.0,
        )
    }

    pub fn kinetic_energy(self: &Universe) -> Energy {
        self.bodies
            .iter()
            .fold(Energy(0.0), |acc, body| acc + Energy(body.mass.0 * body.velocity.0.magnitude().powi(2)))
    }

    // TODO compute kinetic energy
    // TODO compute potential energy
    // TODO compute total energy
}
