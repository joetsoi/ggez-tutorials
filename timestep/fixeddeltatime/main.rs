use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{filesystem, graphics, timer};
use ggez::{Context, GameResult};

const DESIRED_SIMULATION_FPS: u32 = 24;

impl EventHandler for FixedDeltaTime {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_SIMULATION_FPS) {
            self.update_ticks += 1;
            println!(
                "[update] tick: {} \tupdate_tick: {}\tfps: {}",
                timer::ticks(ctx),
                self.update_ticks,
                timer::fps(ctx),
            );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {}", fps));
        println!(
            "[draw] tick: {} \tupdate_tick: {}\tfps: {}",
            timer::ticks(ctx),
            self.update_ticks,
            timer::fps(ctx),
        );
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::BLACK))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

struct FixedDeltaTime {
    update_ticks: u32,
}

impl FixedDeltaTime {
    pub fn new(_ctx: &mut Context) -> FixedDeltaTime {
        FixedDeltaTime { update_ticks: 0 }
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
    let mut vsync_demo = FixedDeltaTime::new(ctx);
    event::run(ctx, event_loop, &mut vsync_demo)
}
