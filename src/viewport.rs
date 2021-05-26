use crate::physics::primitives::{Scalar, Position, Vector2D};
use piston::RenderArgs;
use graphics::math::{transform_pos, Vec2d, transform_vec};

impl From<Vector2D> for Vec2d {
    fn from(vector2d: Vector2D) -> Self {
        [vector2d.x, vector2d.y]
    }
}

pub struct Viewport {
    pub x_min: Scalar,
    pub x_max: Scalar,
    pub y_min: Scalar,
    pub y_max: Scalar,
}

impl Viewport {
    pub fn convert_for_window(self: &Viewport, render_args: &RenderArgs, position: Position) -> (f64, f64) {
        let x_size = (self.x_max - self.x_min).abs();
        let y_size = (self.y_max - self.y_min).abs();

        let [window_x_size, window_y_size] = render_args.window_size;

        let (normalised_x, normalised_y) = (position.0.x / x_size, position.0.y / y_size);

        (normalised_x * window_x_size + (window_x_size / 2.0), normalised_y * window_y_size + (window_y_size / 2.0))
    }
}

mod test {
    use crate::viewport::Viewport;
    use piston::RenderArgs;
    use crate::physics::primitives::{Position, Vector2D};

    #[test]
    fn test_convert_for_window() {
        let viewport = Viewport {
            x_min: -50.0,
            x_max: 50.0,
            y_min: -50.0,
            y_max: 50.0,
        };

        let render_args = RenderArgs {
            ext_dt: 0.0,
            window_size: [200.0, 200.0],
            draw_size: [200, 200],
        };

        assert_eq!(viewport.convert_for_window(&render_args, Position(Vector2D::new(0.0, 0.0))), (100.0, 100.0));
        assert_eq!(viewport.convert_for_window(&render_args, Position(Vector2D::new(25.0, 25.0))), (150.0, 150.0));
    }
}