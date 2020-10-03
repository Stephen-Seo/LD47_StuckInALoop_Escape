use ggez::{Context, GameResult};

pub trait SubEventHandler {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    fn next(&mut self);
    fn finished(&mut self) -> bool;
}
