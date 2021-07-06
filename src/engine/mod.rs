use std::collections::{HashMap, HashSet};

use crate::engine::universe::{Body, Universe};
use crate::physics::collision::collide;
use crate::physics::gravity::Gravity;
use crate::physics::numerical_methods::euler_method;
use crate::physics::primitives::*;
use crate::physics::primitives::*;

pub mod metrics;
pub mod universe;

pub struct Engine {}

impl Engine {
    pub fn step_forward(self: &Engine, universe: &Universe, dt: TemporalDuration) -> Universe {
        let mut new_bodies: Vec<Body> = Vec::with_capacity(universe.bodies.len());
        let mut indexes_of_deleted_bodies: HashSet<usize> = HashSet::new();

        for (object_index, object) in universe.bodies.iter().enumerate() {
            if indexes_of_deleted_bodies.contains(&object_index) {
                continue;
            }

            // TODO this is incorrect when there is a collision, which edits the position too
            let new_position = object.position + object.velocity * dt;

            let mut total_force = Force(Vector2D::zero());

            let mut object_after_all_collisions: Body = *object;

            for (subject_index, subject) in universe.bodies.iter().enumerate() {
                if std::ptr::eq(subject, object)
                    || indexes_of_deleted_bodies.contains(&subject_index)
                {
                    continue;
                }

                let collision_result: Option<Body> =
                    collide(&object_after_all_collisions, subject, dt);

                if collision_result.is_none() {
                    total_force = total_force
                        + universe
                            .gravity
                            .due_to_bodies(&object_after_all_collisions, &subject);
                } else {
                    indexes_of_deleted_bodies.insert(subject_index);
                }

                object_after_all_collisions =
                    collision_result.unwrap_or(object_after_all_collisions);
            }

            let acceleration = total_force / object_after_all_collisions.mass;

            // TODO this should probably be injected
            let new_velocity =
                euler_method::next_velocity(acceleration, object_after_all_collisions.velocity, dt);

            let new_body = Body {
                position: new_position,
                velocity: new_velocity,
                ..(object_after_all_collisions)
            };

            new_bodies.push(new_body)
        }

        Universe {
            bodies: new_bodies,
            ..(*universe)
        }
    }
}
