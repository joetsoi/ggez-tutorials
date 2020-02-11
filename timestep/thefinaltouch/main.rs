use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{filesystem, graphics, timer};
use ggez::{Context, GameResult};

const PHYSICS_SIMULATION_FPS: u32 = 100;
const PHYSICS_DELTA_TIME: f64 = 1.0 / PHYSICS_SIMULATION_FPS as f64;

impl EventHandler for TheFinalTouch {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, PHYSICS_SIMULATION_FPS) {
            let distance = self.velocity_x as f64 * PHYSICS_DELTA_TIME;
            println!("[update] distance {}", distance);
            self.previous_x = self.pos_x;
            self.pos_x = self.pos_x % 800.0 + distance as f32;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.draw_fps_counter(ctx)?;

        let blended_x = self.interpolate(ctx);
        self.draw_circles(ctx, blended_x)?;
        graphics::present(ctx)
    }
}

struct TheFinalTouch {
    previous_x: f32,
    pos_x: f32,
    velocity_x: f32,
}

impl TheFinalTouch {
    pub fn interpolate(&self, ctx: &mut Context) -> f32 {
        let remainder = timer::remaining_update_time(ctx).as_secs_f64();
        let alpha = remainder / PHYSICS_DELTA_TIME;
        let previous_x = if self.pos_x >= self.previous_x {
            self.previous_x
        } else {
            // if we're wrapping round, interpolating in between the two would mean
            // circle would zip from right to left instead of going off screen
            800.0 - self.previous_x
        };
        let blended_x = (self.pos_x * alpha as f32) + (previous_x * (1.0 - alpha as f32));
        blended_x
    }

    pub fn draw_circles(&self, ctx: &mut Context, blended_x: f32) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::BLACK,
        )?;
        println!("{:?} {:?}", self.pos_x, blended_x);
        graphics::draw(ctx, &circle, (Point2::new(self.pos_x, 150.0),))?;
        graphics::draw(ctx, &circle, (Point2::new(blended_x, 380.0),))
    }

    pub fn new(_ctx: &mut Context) -> TheFinalTouch {
        TheFinalTouch {
            previous_x: 0.0,
            pos_x: 0.0,
            velocity_x: 150.0,
        }
    }

    pub fn draw_fps_counter(&self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let delta = timer::delta(ctx);
        let stats_display = graphics::Text::new(format!("FPS: {}, delta: {:?}", fps, delta));
        println!(
            "[draw] ticks: {}\tfps: {}\tdelta: {:?}",
            timer::ticks(ctx),
            fps,
            delta,
        );
        graphics::draw(
            ctx,
            &stats_display,
            (Point2::new(0.0, 0.0), graphics::BLACK),
        )
    }
}

fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("name", "author");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let path = path::PathBuf::from(manifest_dir).join("resources");
        cb = cb.add_resource_path(path);
    }
    let (ctx, event_loop) = &mut cb.build()?;
    println!("{:#?}", filesystem::read_config(ctx));
    let mut vsync_demo = TheFinalTouch::new(ctx);
    event::run(ctx, event_loop, &mut vsync_demo)
}
