use std::cmp::Ordering;
use std::time::Duration;
use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{filesystem, graphics, timer};
use ggez::{Context, GameResult};

impl EventHandler for SemiFixedTimeStep {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut frame_time = timer::delta(ctx).as_secs_f64();

        while frame_time > 0.0 {
            let cmp = frame_time.partial_cmp(&self.dt).expect("float NaN error");
            let delta_time: f64 = if let Ordering::Less = cmp {
                frame_time
            } else {
                self.dt
            };
            self.simulate(delta_time);
            frame_time -= delta_time;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.draw_fps_counter(ctx)?;
        self.draw_circle(ctx, self.pos_x)?;
        graphics::present(ctx)?;
        // timer::sleep(Duration::from_secs(2));
        Ok(())
    }
}

struct SemiFixedTimeStep {
    pos_x: f32,
    velocity_x: f32,
    dt: f64, // upper bound for our time step;
}

impl SemiFixedTimeStep {
    pub fn new(_ctx: &mut Context) -> SemiFixedTimeStep {
        SemiFixedTimeStep {
            pos_x: 0.0,
            velocity_x: 60.0,
            dt: 1.0f64 / 60.0f64,
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

    pub fn draw_circle(&self, ctx: &mut Context, x: f32) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (Point2::new(x, 380.0),))
    }

    pub fn simulate(&mut self, time: f64) {
        let distance = self.velocity_x as f64 * time;
        self.pos_x = self.pos_x % 800.0 + distance as f32;
        println!("[update] distance {}", distance);
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
    let mut vsync_demo = SemiFixedTimeStep::new(ctx);
    event::run(ctx, event_loop, &mut vsync_demo)
}
