use core::ops;
use std::ops::Mul;

use crate::physics::primitives::Scalar;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2D {
    x: Scalar,
    y: Scalar,
}

impl Vector2D {
    pub fn zero() -> Vector2D {
        Vector2D { x: 0.0, y: 0.0 }
    }

    pub fn new(x: Scalar, y: Scalar) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn magnitude(self: Vector2D) -> Scalar {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn unit(self: Vector2D) -> Vector2D {
        let magnitude = self.magnitude();

        Vector2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}

impl ops::Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<Scalar> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vector2D> for Scalar {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::primitives::vector::Vector2D;

    #[test]
    fn vector_magnitude() {
        assert_eq!(Vector2D::new(5.0, 12.0).magnitude(), 13.0)
    }

    #[test]
    fn vector_addition() {
        assert_eq!(
            Vector2D::new(1.0, 2.0) + Vector2D::new(-4.0, 6.0),
            Vector2D::new(-3.0, 8.0)
        )
    }

    #[test]
    fn vector_subtraction() {
        assert_eq!(
            Vector2D::new(1.0, 2.0) - Vector2D::new(-4.0, 6.0),
            Vector2D::new(5.0, -4.0)
        )
    }
}
