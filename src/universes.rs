use crate::engine::universe::{Body, Universe};
use crate::physics::gravity::{GravitationalConstant, Gravity};
use crate::physics::primitives::*;
use core::iter;
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

pub fn pluto_and_charon() -> Universe {
    let g = GravitationalConstant::UNIVERSAL;

    let pluto = Body {
        mass: Mass(1.303e22),
        position: Position(Vector2D::zero()),
        velocity: Velocity(Vector2D::zero()),
    };

    let charon = Body {
        mass: Mass(1.586e21),
        position: Position(Vector2D::new(19587000.0, 0.0)),
        velocity: Velocity(Vector2D::new(0.0, 210.0)),
    };

    Universe {
        gravity: Gravity {
            gravitational_constant: g,
        },
        bodies: vec![pluto, charon],
        age: TemporalDuration(0.0),
    }
}

pub fn random(
    n_bodies: u16,
    mass_distribution: impl Distribution<Scalar>,
    position_distribution: impl Distribution<Vector2D>,
    velocity_distribution: impl Distribution<Vector2D>,
) -> Universe {
    let masses = mass_distribution.sample_iter(rand::thread_rng());
    let positions = position_distribution.sample_iter(rand::thread_rng());
    let velocities = velocity_distribution.sample_iter(rand::thread_rng());

    let bodies = masses
        .zip(positions)
        .zip(velocities)
        .take(n_bodies as usize)
        .map(|((mass, position_vector), velocity_vector)| Body {
            mass: Mass(mass),
            position: Position(position_vector),
            velocity: Velocity(velocity_vector),
        })
        .collect();

    Universe {
        gravity: Gravity::UNIVERSAL,
        bodies,
        age: TemporalDuration(0.0),
    }
}

pub struct BoxedVector2DDistribution {
    pub x_min: Scalar,
    pub x_max: Scalar,
    pub y_min: Scalar,
    pub y_max: Scalar,
}

impl Distribution<Vector2D> for BoxedVector2DDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2D {
        Vector2D {
            x: rng.gen_range((self.x_min..self.x_max)),
            y: rng.gen_range((self.y_min..self.y_max)),
        }
    }
}

pub struct CircularVector2DDistribution {
    pub magnitude_min: Scalar,
    pub magnitude_max: Scalar,
}

impl Distribution<Vector2D> for CircularVector2DDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2D {
        let theta: Scalar = rng.gen_range((0.0..2.0 * std::f64::consts::PI));
        let r: Scalar = rng.gen_range((self.magnitude_min..self.magnitude_max));

        Vector2D {
            x: r * Scalar::cos(theta),
            y: r * Scalar::sin(theta),
        }
    }
}
