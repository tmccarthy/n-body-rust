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

        let position_relative_to_viewport_normalised: Position = Position(Vector2D {
            x: position.0.x / x_size,
            y: position.0.y / y_size,
        });

        let window_transform_matrix = graphics::math::invert(render_args.viewport().abs_transform());

        let graphics_library_vec = Vec2d::from(position_relative_to_viewport_normalised.0);

        let [window_x, window_y] = transform_vec(window_transform_matrix, graphics_library_vec);

        (window_x, window_y)
    }
}
