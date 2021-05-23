use crate::engine::{Universe, Body, BodyId};
use crate::physics::primitives::*;
use crate::physics::gravity::{GravitationalConstant, Gravity};

pub fn pluto_and_charon() -> Universe {
    let g = GravitationalConstant::UNIVERSAL;

    let pluto = Body {
        id: BodyId(0),
        mass: Mass(1.303e22),
        position: Position(Vector2D::zero()),
        velocity: Velocity(Vector2D::zero()),
    };

    let charon = Body {
        id: BodyId(1),
        mass: Mass(1.586e21),
        position: Position(Vector2D::new(19587000.0, 0.0)),
        velocity: Velocity(Vector2D::new(0.0, 210.0)),
    };

    Universe {
        gravity: Gravity { gravitational_constant: g },
        bodies: vec![pluto, charon]
    }
}