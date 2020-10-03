use std::cell::RefCell;
use std::rc::Rc;

use ggez::event::EventHandler;
use ggez::graphics::{self, Font, Text};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use super::Scene;
use crate::player::Player;

pub struct MainScene {
    font: Font,
    player: Rc<RefCell<Player>>,
    finished: bool,
    current_text: Text,
    final_text: String,
    text_idx: usize,
}

impl MainScene {
    pub fn new(_ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Self {
        Self {
            font,
            player,
            finished: false,
            current_text: Text::new("".to_owned()),
            final_text: String::new(),
            text_idx: 0usize,
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

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
    }
}

impl Scene for MainScene {
    fn finished(&self) -> bool {
        self.finished
    }
}
