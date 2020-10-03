use std::rc::Rc;
use std::cell::RefCell;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Font};
use ggez::event::EventHandler;
use ggez::input::mouse::MouseButton;

use crate::player::Player;
use super::Scene;

pub struct MainScene {
    font: Font,
    player: Rc<RefCell<Player>>,
    finished: bool,
}

impl MainScene {
    pub fn new(_ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Self {
        Self {
            font,
            player,
            finished: false,
        }
    }

    pub fn new_boxed(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Box<Self> {
        Box::new(Self::new(ctx, font, player))
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player.borrow_mut().draw(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
    }
}

impl Scene for MainScene {
    fn finished(&self) -> bool {
        self.finished
    }
}
