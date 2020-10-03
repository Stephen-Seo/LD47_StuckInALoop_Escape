use crate::subeventhandler::SubEventHandler;
use ggez::{Context, GameResult};

pub struct GameStartScene {}

impl GameStartScene {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {}
    }

    pub fn new_boxed(ctx: &mut Context) -> Box<Self> {
        Box::new(Self::new(ctx))
    }
}

impl SubEventHandler for GameStartScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn next(&mut self) {}

    fn finished(&mut self) -> bool {
        false
    }
}
