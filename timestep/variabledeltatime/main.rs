use std::time::Duration;
use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{filesystem, graphics, timer};
use ggez::{Context, GameResult};

impl EventHandler for VariableDeltaTime {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.simulate(timer::delta(ctx));
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

struct VariableDeltaTime {
    pos_x: f32,
    velocity_x: f32,
}

impl VariableDeltaTime {
    pub fn new(_ctx: &mut Context) -> VariableDeltaTime {
        VariableDeltaTime {
            pos_x: 0.0,
            velocity_x: 60.0,
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

    pub fn simulate(&mut self, time: Duration) {
        let distance = self.velocity_x * time.as_secs_f32();
        println!(
            "[update] distance {}\ttime: {}",
            distance,
            time.as_secs_f64()
        );
        self.pos_x = self.pos_x % 800.0 + distance;
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
    let mut vsync_demo = VariableDeltaTime::new(ctx);
    event::run(ctx, event_loop, &mut vsync_demo)
}
