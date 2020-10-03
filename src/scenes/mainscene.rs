use std::cell::RefCell;
use std::rc::Rc;

use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam, Font, Image, Text};
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
    music: Source,
    pod_image: Image,
    pod_empty_image: Image,
}

impl MainScene {
    pub fn new(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Self {
        let mut music = Source::new(ctx, "/music00.ogg").unwrap();
        music.set_repeat(true);
        //        music.play().unwrap();
        Self {
            font,
            player,
            finished: false,
            current_text: Text::new("".to_owned()),
            final_text: String::new(),
            text_idx: 0usize,
            music,
            pod_image: Image::new(ctx, "/stasis_pod.png").unwrap(),
            pod_empty_image: Image::new(ctx, "/stasis_pod_empty.png").unwrap(),
        }
    }

    pub fn new_boxed(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Box<Self> {
        Box::new(Self::new(ctx, font, player))
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player.borrow_mut().update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player.borrow_mut().draw(ctx)?;
        graphics::draw(
            ctx,
            &self.pod_image,
            DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
        )?;
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
