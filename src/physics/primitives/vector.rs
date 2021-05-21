use core::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn magnitude(self: Vector2D) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::physics::primitives::vector::Vector2D;

    #[test]
    fn vector_magnitude() {
        assert_eq!(Vector2D::new(5f64, 12f64).magnitude(), 13f64)
    }

    #[test]
    fn vector_addition() {
        assert_eq!(Vector2D::new(1f64, 2f64) + Vector2D::new(-4f64, 6f64), Vector2D::new(-3f64, 8f64))
    }

    #[test]
    fn vector_subtraction() {
        assert_eq!(Vector2D::new(1f64, 2f64) - Vector2D::new(-4f64, 6f64), Vector2D::new(5f64, -4f64))
    }
}
