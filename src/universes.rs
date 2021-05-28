use crate::engine::{Body, BodyId, Universe};
use crate::physics::gravity::{GravitationalConstant, Gravity};
use crate::physics::primitives::*;

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
        gravity: Gravity {
            gravitational_constant: g,
        },
        bodies: vec![pluto, charon],
    }
}

pub fn random<FW, FP>(n_bodies: u16, gen_mass: FW, gen_position: FP) -> Universe
where
    FW: Fn() -> Mass,
    FP: Fn() -> Position,
{
    todo!()
}
