use std::{env, path};

use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{filesystem, graphics, timer};
use ggez::{Context, GameResult};

impl EventHandler for VSync {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!(
            "[update] ticks: {}\tfps: {}\tdelta: {:?}",
            timer::ticks(ctx),
            timer::fps(ctx),
            timer::delta(ctx)
        );
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {}", fps));
        println!(
            "[draw] ticks: {}\tfps: {}\tdelta: {:?}",
            timer::ticks(ctx),
            fps,
            timer::delta(ctx)
        );
        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &fps_display, (Point2::new(0.0, 0.0), graphics::BLACK))?;
        graphics::present(ctx)
    }
}

struct VSync {}

impl VSync {
    pub fn new(_ctx: &mut Context) -> VSync {
        VSync {}
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
    let mut vsync_demo = VSync::new(ctx);
    event::run(ctx, event_loop, &mut vsync_demo)
}
