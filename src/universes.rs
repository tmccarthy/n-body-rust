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
    }
}

pub fn random(
    n_bodies: u16,
    mass_distribution: impl Distribution<Scalar>,
    position_distribution: impl Distribution<Vector2D>,
) -> Universe {
    let masses = mass_distribution.sample_iter(rand::thread_rng());
    let positions = position_distribution.sample_iter(rand::thread_rng());
    let velocities = iter::repeat(Velocity(Vector2D::zero()));

    let bodies = masses
        .zip(positions)
        .zip(velocities)
        .take(n_bodies as usize)
        .map(|((mass, position_vector), velocity)| Body {
            mass: Mass(mass),
            position: Position(position_vector),
            velocity,
        })
        .collect();

    Universe {
        gravity: Gravity::UNIVERSAL,
        bodies,
    }
}

pub struct Vector2DDistribution {
    pub x_min: Scalar,
    pub x_max: Scalar,
    pub y_min: Scalar,
    pub y_max: Scalar,
}

impl Distribution<Vector2D> for Vector2DDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector2D {
        Vector2D {
            x: rng.gen_range((self.x_min..self.x_max)),
            y: rng.gen_range((self.y_min..self.y_max)),
        }
    }
}
