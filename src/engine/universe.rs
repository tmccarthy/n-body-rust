use crate::physics::gravity::*;
use crate::physics::primitives::*;

// TODO do we need copy/clone here?
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
    pub fn due_to_bodies(self: &Gravity, object: &Body, subject: &Body) -> Force {
        self.due_to(object.position, object.mass, subject.position, subject.mass)
    }
}

impl Universe {
    pub fn add_body(&mut self, body: Body) -> () {
        self.bodies.push(body);
    }

    // TODO the following should really be in the physics package

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
        self.bodies.iter().fold(Energy(0.0), |acc, body| {
            acc + Energy(body.mass.0 * body.velocity.0.magnitude().powi(2))
        })
    }

    // TODO compute kinetic energy
    // TODO compute potential energy
    // TODO compute total energy
}
