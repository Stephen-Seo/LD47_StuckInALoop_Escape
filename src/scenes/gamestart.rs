use ggez::event::EventHandler;
use ggez::{Context, GameResult};

use super::Scene;

pub struct GameStartScene {}

impl GameStartScene {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {}
    }

    pub fn new_boxed(ctx: &mut Context) -> Box<Self> {
        Box::new(Self::new(ctx))
    }
}

impl EventHandler for GameStartScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl Scene for GameStartScene {
    fn finished(&self) -> bool {
        false
    }
}
