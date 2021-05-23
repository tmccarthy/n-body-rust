use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::engine::Universe;
use crate::physics::primitives::{Scalar, TemporalDuration};

mod engine;
mod physics;
mod universes;

struct Viewport {
    x_min: Scalar,
    x_max: Scalar,
    y_min: Scalar,
    y_max: Scalar,
}



fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("n-body", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());

    let mut universe: Universe = universes::pluto_and_charon();
    let time_scale: Scalar = 1e4;

    let viewport: Viewport = todo!();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            render(distance_scale, &universe, &args);
        }

        if let Some(args) = e.update_args() {
            ui_driven_update(time_scale, &mut universe, &args);
        }
    }
}

fn render(viewport: &Viewport, universe: &Universe, args: &RenderArgs) -> () {
    todo!()
}

fn ui_driven_update(time_scale: Scalar, universe: &mut Universe, args: &UpdateArgs) -> () {
    let ui_dt = TemporalDuration(args.dt);
    let dt = time_scale * ui_dt;
    universe.step_forward(dt);
}