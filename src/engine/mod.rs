use std::collections::{HashMap, HashSet};

use crate::engine::universe::{Body, Universe};
use crate::physics::collision::collide;
use crate::physics::gravity::Gravity;
use crate::physics::numerical_methods::{euler_method, OdeAlgorithm};
use crate::physics::primitives::*;
use crate::physics::primitives::*;
use chashmap::CHashMap;
use rayon::prelude::*;

pub mod metrics;
pub mod universe;

pub struct Engine<A: OdeAlgorithm<Vector2D, Scalar>> {
    pub numerical_method: A,
}

impl<A: OdeAlgorithm<Vector2D, Scalar>> Engine<A> {
    pub fn step_forward(self: &Engine<A>, universe: &Universe, dt: TemporalDuration) -> Universe {
        let mut indexes_of_deleted_bodies: CHashMap<usize, ()> = CHashMap::new();

        let new_bodies = universe
            .bodies
            .par_iter()
            .enumerate()
            .filter_map(|(object_index, object)| {
                if indexes_of_deleted_bodies.contains_key(&object_index) {
                    return None;
                }

                // TODO this is incorrect when there is a collision, which edits the position too
                let new_position = object.position + object.velocity * dt;

                let mut total_force = Force(Vector2D::zero());

                let mut object_after_all_collisions: Body = *object;

                for (subject_index, subject) in universe.bodies.iter().enumerate() {
                    if std::ptr::eq(subject, object)
                        || indexes_of_deleted_bodies.contains_key(&subject_index)
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
                        indexes_of_deleted_bodies.insert(subject_index, ());
                    }

                    object_after_all_collisions =
                        collision_result.unwrap_or(object_after_all_collisions);
                }

                let acceleration = total_force / object_after_all_collisions.mass;

                let new_velocity = Velocity(self.numerical_method.next_y(
                    |_, _| acceleration.0,
                    object_after_all_collisions.velocity.0,
                    universe.age.0,
                    dt.0,
                ));

                let new_body = Body {
                    position: new_position,
                    velocity: new_velocity,
                    ..(object_after_all_collisions)
                };

                Some(new_body)
            })
            .collect();

        Universe {
            bodies: new_bodies,
            age: universe.age + dt,
            ..(*universe)
        }
    }
}
