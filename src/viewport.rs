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
    pub fn zero() -> Viewport {
        Viewport::around_origin(0.0, 0.0)
    }

    pub fn around_origin(x_size: Scalar, y_size: Scalar) -> Viewport {
        Viewport::around(Position(Vector2D::zero()), x_size, y_size)
    }

    pub fn square_around(centre: Position, size: Scalar) -> Viewport {
        Viewport::around(centre, size, size)
    }

    pub fn around(centre: Position, x_size: Scalar, y_size: Scalar) -> Viewport {
        Viewport {
            x_min: centre.0.x - x_size / 2.0,
            x_max: centre.0.x + x_size / 2.0,
            y_min: centre.0.y - y_size / 2.0,
            y_max: centre.0.y + y_size / 2.0,
        }
    }

    pub fn convert_for_window(self: &Viewport, render_args: &RenderArgs, position: Position) -> (f64, f64) {
        let window_origin_in_viewport = (self.x_min, self.y_max);
        let window_bottom_right_in_viewport = (self.x_max, self.y_min);
        let window_size_in_viewport = (
            (window_bottom_right_in_viewport.0 - window_origin_in_viewport.0).abs(),
            (window_bottom_right_in_viewport.1 - window_origin_in_viewport.1).abs(),
        );

        let normalised_position_within_viewport = (
            (position.0.x - window_origin_in_viewport.0) / window_size_in_viewport.0,
            -1.0 * ((position.0.y - window_origin_in_viewport.1) / window_size_in_viewport.1),
        );

        let [window_x_size, window_y_size] = render_args.window_size;

        (
            normalised_position_within_viewport.0 * window_x_size,
            normalised_position_within_viewport.1 * window_y_size,
        )
    }
}

mod test {
    use crate::viewport::Viewport;
    use piston::RenderArgs;
    use crate::physics::primitives::{Position, Vector2D};

    #[test]
    fn test_convert_for_window_viewport_around_origin() {
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
        assert_eq!(viewport.convert_for_window(&render_args, Position(Vector2D::new(25.0, 25.0))), (150.0, 50.0));
    }

    #[test]
    fn test_convert_for_window_viewport_positive() {
        let viewport = Viewport {
            x_min: 10.0,
            x_max: 50.0,
            y_min: 10.0,
            y_max: 50.0,
        };

        let render_args = RenderArgs {
            ext_dt: 0.0,
            window_size: [200.0, 200.0],
            draw_size: [200, 200],
        };

        assert_eq!(viewport.convert_for_window(&render_args, Position(Vector2D::new(0.0, 0.0))), (-50.0, 250.0));
        assert_eq!(viewport.convert_for_window(&render_args, Position(Vector2D::new(25.0, 25.0))), (75.0, 125.0));
    }
}