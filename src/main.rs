extern crate graphics;

use std::cmp::{max, min};

use glutin_window::GlutinWindow as Window;
use graphics::ellipse::circle;
use graphics::rectangle::centered_square;
use graphics::{clear, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::engine::{Body, BodyId, Universe};
use crate::physics::primitives::{Mass, Position, Scalar, TemporalDuration, Vector2D, Velocity};
use crate::universes::Vector2DDistribution;
use crate::viewport::Viewport;
use rand::distributions::Uniform;

mod engine;
mod physics;
mod universes;
mod viewport;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("n-body", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());

    let mut graphics = GlGraphics::new(opengl);

    let mut universe: Universe = universes::random(
        100,
        Uniform::new(0.0, 1e21),
        Vector2DDistribution {
            x_min: -1e8,
            x_max: 1e8,
            y_min: -1e8,
            y_max: 1e8,
        },
    );
    let time_scale: Scalar = 1e4;

    let viewport_size = 4e8;

    let mut viewport: Viewport = update_viewport_for(&universe, viewport_size);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            render(&mut graphics, &viewport, &universe, &args);
        }

        if let Some(args) = e.update_args() {
            universe = ui_driven_update(time_scale, &universe, &args);
            viewport = update_viewport_for(&universe, viewport_size);
        }
    }
}

fn render(
    graphics: &mut GlGraphics,
    viewport: &Viewport,
    universe: &Universe,
    args: &RenderArgs,
) -> () {
    graphics.draw(args.viewport(), |context, graphics| {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        for body in &universe.bodies {
            let (window_x, window_y) = viewport.convert_for_window(args, body.position);

            let circle = centered_square(window_x, window_y, 10.0);

            graphics::ellipse([1.0, 1.0, 1.0, 1.0], circle, context.transform, graphics);
        }
    });
}

fn ui_driven_update(time_scale: Scalar, old_universe: &Universe, args: &UpdateArgs) -> Universe {
    let ui_dt = TemporalDuration(args.dt);
    let dt = ui_dt * time_scale;
    old_universe.step_forward(dt)
}

fn update_viewport_for(universe: &Universe, viewport_size: Scalar) -> Viewport {
    let centre_of_mass = universe.centre_of_mass();

    Viewport::square_around(centre_of_mass, viewport_size)
}
