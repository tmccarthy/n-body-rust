extern crate graphics;

use std::cmp::{max, min};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

use font_kit::font::Font;
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use glutin_window::GlutinWindow as Window;
use graphics::{CharacterCache, clear, Context, Graphics, types};
use graphics::ellipse::circle;
use graphics::rectangle::centered_square;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture};
use opengl_graphics::TextureSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::distributions::Uniform;

use crate::engine::{Body, Universe};
use crate::engine::metrics::Metric;
use crate::graphics::Transformed;
use crate::physics::primitives::{Mass, Position, Scalar, TemporalDuration, Vector2D, Velocity};
use crate::universes::Vector2DDistribution;
use crate::viewport::Viewport;

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
    let mut charachter_cache: GlyphCache = make_character_cache().unwrap();

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
            render(&mut graphics, &mut charachter_cache, &viewport, &universe, &args);
        }

        if let Some(args) = e.update_args() {
            universe = ui_driven_update(time_scale, &universe, &args);
            viewport = update_viewport_for(&universe, viewport_size);
        }
    }
}

fn render<C: CharacterCache<Texture = Texture>>(
    graphics: &mut GlGraphics,
    character_cache: &mut C,
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

        draw_metrics(graphics, context, character_cache, universe, [Metric::NumBodies, Metric::KineticEnergy])
            .unwrap();

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

fn draw_metrics<C: CharacterCache<Texture = Texture>, const N: usize>(
    graphics: &mut GlGraphics,
    context: Context,
    character_cache: &mut C,
    universe: &Universe,
    metrics: [Metric; N],
) -> Result<(), ()> {
    let complete_string = metrics.iter()
        .map(|m| format!("{}: {}", m.symbol(), m.compute_from(universe)))
        .collect::<Vec<String>>()
        .join("\n\r");

    graphics::text(graphics::color::WHITE, 10, &complete_string, character_cache, context.trans(10.0, 10.0).transform, graphics)
        .map_err(|_| ())
}

fn make_character_cache<'a>() -> Result<GlyphCache<'a>, CharCacheError> {
    let font_handle: Handle = SystemSource::new()
        .select_by_postscript_name("ArialMT")
        .map_err(CharCacheError::SelectFontError)?;

    let font_path = match font_handle {
        Handle::Path { path, .. } => Ok(path),
        Handle::Memory { .. } => Err(CharCacheError::FontInMemoryError),
    }?;

    let character_cache = GlyphCache::new(font_path, (), TextureSettings::new())
        .map_err(|_| CharCacheError::GlyphCacheError)?;

    Ok(character_cache)
}

#[derive(Debug)]
enum CharCacheError {
    SelectFontError(font_kit::error::SelectionError),
    FontInMemoryError,
    CopyFontDataError,
    GlyphCacheError,
}

impl Display for CharCacheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for CharCacheError {}