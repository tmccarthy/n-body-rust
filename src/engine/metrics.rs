use std::fmt::Debug;

use crate::engine::Universe;
use crate::physics::primitives::{Vector2D, Scalar};

pub enum Metric {
    NumBodies,
    Momentum,
    KineticEnergy,
}

impl Metric {
    pub fn compute_from(self: &Metric, universe: &Universe) -> String {
        match self {
            Metric::NumBodies => universe.bodies.len().to_string(),
            Metric::Momentum => universe.momentum().0.to_string(),
            Metric::KineticEnergy => universe.kinetic_energy().0.to_string(),
        }
    }

    pub fn symbol(self: &Metric) -> &'static str {
        match self {
            Metric::NumBodies => "n",
            Metric::Momentum => "Σp̃",
            Metric::KineticEnergy => "ΣEₖ",
        }
    }
}
