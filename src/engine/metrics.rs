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

//
// pub trait Metric<T: ToString + ?Sized> {
//     const SYMBOL: &'static str;
//     fn compute_from(universe: Universe) -> T;
// }
//
// pub struct NumBodies;
//
// impl Metric<usize> for NumBodies {
//     const SYMBOL: &'static str = "n";
//
//     fn compute_from<'a>(universe: Universe) -> usize {
//         universe.bodies.len()
//     }
// }
//
// pub struct Momentum;
//
// impl Metric<Vector2D> for Momentum {
//     const SYMBOL: &'static str = "Σp̃";
//
//     fn compute_from(universe: Universe) -> Vector2D {
//         universe.momentum().0
//     }
// }
//
// pub struct KineticEnergy;
//
// impl Metric<Scalar> for KineticEnergy {
//     const SYMBOL: &'static str = "ΣEₖ";
//
//     fn compute_from(universe: Universe) -> Scalar {
//         universe.kinetic_energy().0
//     }
// }