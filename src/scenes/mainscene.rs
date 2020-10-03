use std::cell::RefCell;
use std::rc::Rc;

use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Image, Mesh, Rect, Text};
use ggez::input::mouse::MouseButton;
use ggez::timer::delta;
use ggez::{Context, GameResult};

use super::Scene;
use crate::player::Player;

const DARKNESS_PAN_RATE: f32 = 50f32;

enum State {
    InPod_InDarkness,
}

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
    ground_rect: Rect,
    state: State,
    darkness_image: Image,
    darkness_yoffset: f32,
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
            ground_rect: Rect::new(0f32, 550f32, 800f32, 50f32),
            state: State::InPod_InDarkness,
            darkness_image: Image::new(ctx, "/darkness.png").unwrap(),
            darkness_yoffset: 0f32,
        }
    }

    pub fn new_boxed(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Box<Self> {
        Box::new(Self::new(ctx, font, player))
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            State::InPod_InDarkness => {
                let mut player = self.player.borrow_mut();
                player.x = 520f32;
                player.y = 350f32;
                player.rot = 0.78f32;
                if self.darkness_yoffset > -400f32 {
                    self.darkness_yoffset -= delta(ctx).as_secs_f32() * DARKNESS_PAN_RATE;
                }
            }
        }
        self.player.borrow_mut().update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            State::InPod_InDarkness => {
                graphics::draw(
                    ctx,
                    &self.pod_image,
                    DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                )?;
                self.player.borrow_mut().draw(ctx)?;
            }
        }

        let ground_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.ground_rect,
            Color::from_rgb(0x49, 0x49, 0x49),
        )?;
        graphics::draw(ctx, &ground_mesh, DrawParam::new())?;

        graphics::draw(
            ctx,
            &self.darkness_image,
            DrawParam::new().dest([0f32, self.darkness_yoffset]),
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
