use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics;

pub struct Game {
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game { }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        graphics::present(ctx)
    }
}
