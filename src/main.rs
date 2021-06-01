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
use std::ops::Range;

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
    let min_mass = universe
        .bodies
        .iter()
        .map(|b| b.mass)
        .reduce(|left, right| Mass(Scalar::min(left.0, right.0)))
        .unwrap_or(Mass(0.0));
    let max_mass = universe
        .bodies
        .iter()
        .map(|b| b.mass)
        .reduce(|left, right| Mass(Scalar::max(left.0, right.0)))
        .unwrap_or(Mass(0.0));

    let mass_range = (min_mass..max_mass);

    graphics.draw(args.viewport(), |context, graphics| {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        for body in &universe.bodies {
            let (window_x, window_y) = viewport.convert_for_window(args, body.position);

            let circle =
                centered_square(window_x, window_y, scale_radius_by(&mass_range, body.mass));

            graphics::ellipse([1.0, 1.0, 1.0, 1.0], circle, context.transform, graphics);
        }
    });
}

fn scale_radius_by(all_masses: &Range<Mass>, mass: Mass) -> graphics::math::Scalar {
    const MAX_RADIUS: graphics::math::Scalar = 5.0;
    const MIN_RADIUS: graphics::math::Scalar = 1.0;

    ((mass.0 - all_masses.start.0) / (all_masses.end.0 - all_masses.start.0))
        * (MAX_RADIUS - MIN_RADIUS)
        + MIN_RADIUS
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
