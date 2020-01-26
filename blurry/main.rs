//! The simplest possible example that does something.
use std::{env, path};


use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra;
use ggez::{Context, GameResult};

struct MainState {
    image: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            image: graphics::Image::new(ctx, "/character.png")?,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::new().scale(nalgebra::Vector2::new(3.0, 3.0)),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("pixel art example", "ggez");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let path = path::PathBuf::from(manifest_dir).join("resources");
        cb = cb.add_resource_path(path);
    }
    let (ctx, event_loop) = &mut cb.build()?;
    // graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
