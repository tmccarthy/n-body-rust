use crate::physics::primitives::{Scalar, Vector2D};

pub mod euler_method;

pub trait OdeAlgorithm<Y, T> {
    fn next_y<F>(self: &Self, y_prime: F, y_0: Y, t_0: T, h: T) -> Y
    where
        F: Fn(T, Y) -> Y;
}

pub struct EulerMethod;

impl OdeAlgorithm<Vector2D, Scalar> for EulerMethod {
    fn next_y<F>(self: &Self, y_prime: F, y_0: Vector2D, t_0: Scalar, h: f64) -> Vector2D
    where
        F: Fn(Scalar, Vector2D) -> Vector2D,
    {
        y_0 + (y_prime(t_0, y_0) * h)
    }
}
