extern crate graphics;

use glutin_window::GlutinWindow as Window;
use graphics::{clear, Graphics};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use crate::engine::Universe;
use crate::physics::primitives::{Scalar, TemporalDuration};
use crate::viewport::{Viewport};
use graphics::ellipse::circle;
use graphics::rectangle::centered_square;

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

    let mut universe: Universe = universes::pluto_and_charon();
    let time_scale: Scalar = 1e6;

    let viewport: Viewport = Viewport {
        x_min: -20000000.0,
        x_max: 20000000.0,
        y_min: -20000000.0,
        y_max: 20000000.0,
    };

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            render(&mut graphics, &viewport, &universe, &args);
        }

        if let Some(args) = e.update_args() {
            ui_driven_update(time_scale, &universe, &args);
        }
    }
}

fn render(graphics: & mut GlGraphics, viewport: &Viewport, universe: &Universe, args: &RenderArgs) -> () {
    graphics.draw(args.viewport(), |canvas, graphics| {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        for body in &universe.bodies {
            let (window_x, window_y) = viewport.convert_for_window(args, body.position);

            let circle = centered_square(window_x, window_y, 40.0);

            graphics::ellipse([1.0, 1.0, 1.0, 1.0], circle, canvas.transform, graphics);
        }
    });
}

fn ui_driven_update(time_scale: Scalar, universe: &Universe, args: &UpdateArgs) -> () {
    let ui_dt = TemporalDuration(args.dt);
    let dt = ui_dt * time_scale;
    universe.step_forward(dt);
}