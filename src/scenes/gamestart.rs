use std::cell::RefCell;
use std::rc::Rc;

use ggez::event::EventHandler;
use ggez::graphics::{
    self, Color, DrawMode, DrawParam, Font, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use super::Scene;
use crate::player::Player;

pub struct GameStartScene {
    font: Font,
    finished: bool,
    color_pale: Color,
    color_white: Color,
    color_brown: Color,
    color_black: Color,
    color_red: Color,
    color_green: Color,
    color_blue: Color,
    pick_color_text: Text,
    player: Rc<RefCell<Player>>,
    drawed_loading_text: bool,
}

impl GameStartScene {
    pub fn new(_ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Self {
        let mut pick_color_text: Text = Text::new(
            TextFragment::new("Pick your color").color(Color::from_rgb(0xff, 0xff, 0xff)),
        );
        pick_color_text.set_font(font, PxScale::from(32f32));
        Self {
            font,
            finished: false,
            color_pale: Color::from_rgb(0xfd, 0xd9, 0xbd),
            color_white: Color::from_rgb(0xfd, 0xba, 0x85),
            color_brown: Color::from_rgb(0xb5, 0x72, 0x37),
            color_black: Color::from_rgb(0x4b, 0x28, 0x0a),
            color_red: Color::from_rgb(0xff, 0, 0),
            color_green: Color::from_rgb(0, 0xff, 0),
            color_blue: Color::from_rgb(0, 0, 0xff),
            pick_color_text,
            player,
            drawed_loading_text: false,
        }
    }

    pub fn new_boxed(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Box<Self> {
        Box::new(Self::new(ctx, font, player))
    }
}

impl EventHandler for GameStartScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let text_width = self.pick_color_text.width(ctx) as f32 / 2f32;
        graphics::draw(
            ctx,
            &self.pick_color_text,
            DrawParam::new().dest([400f32 - text_width, 150f32]),
        )?;

        let pale_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_pale,
        )?;
        graphics::draw(
            ctx,
            &pale_mesh,
            DrawParam::new().dest([400f32 - 128f32 - 64f32 - 64f32, 200f32]),
        )?;

        let white_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_white,
        )?;
        graphics::draw(
            ctx,
            &white_mesh,
            DrawParam::new().dest([400f32 - 64f32 - 64f32, 200f32]),
        )?;

        let brown_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_brown,
        )?;
        graphics::draw(ctx, &brown_mesh, DrawParam::new().dest([400f32, 200f32]))?;

        let black_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_black,
        )?;
        graphics::draw(
            ctx,
            &black_mesh,
            DrawParam::new().dest([400f32 + 128f32, 200f32]),
        )?;

        let red_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_red,
        )?;
        graphics::draw(
            ctx,
            &red_mesh,
            DrawParam::new().dest([400f32 - 128f32 - 64f32, 328f32]),
        )?;

        let green_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_green,
        )?;
        graphics::draw(
            ctx,
            &green_mesh,
            DrawParam::new().dest([400f32 - 64f32, 328f32]),
        )?;

        let blue_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0f32, 0f32, 128f32, 128f32),
            self.color_blue,
        )?;
        graphics::draw(
            ctx,
            &blue_mesh,
            DrawParam::new().dest([400f32 + 128f32 - 64f32, 328f32]),
        )?;

        if self.finished {
            self.pick_color_text = Text::new("Loading...");
            self.pick_color_text
                .set_font(self.font, PxScale::from(32f32));
            let text_width = self.pick_color_text.width(ctx) as f32 / 2f32;
            graphics::draw(
                ctx,
                &self.pick_color_text,
                DrawParam::new().dest([400f32 - text_width, 520f32]),
            )?;
            self.drawed_loading_text = true;
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if y > 200f32 && y < 200f32 + 128f32 {
            if x > 400f32 - 256f32 && x < 400f32 - 128f32 {
                self.player.borrow_mut().color = self.color_pale;
                self.finished = true;
            } else if x > 400f32 - 128f32 && x < 400f32 {
                self.player.borrow_mut().color = self.color_white;
                self.finished = true;
            } else if x > 400f32 && x < 400f32 + 128f32 {
                self.player.borrow_mut().color = self.color_brown;
                self.finished = true;
            } else if x > 400f32 + 128f32 && x < 400f32 + 256f32 {
                self.player.borrow_mut().color = self.color_black;
                self.finished = true;
            }
        } else if y > 328f32 && y < 328f32 + 128f32 {
            if x > 400f32 - 128f32 - 64f32 && x < 400f32 - 64f32 {
                self.player.borrow_mut().color = self.color_red;
                self.finished = true;
            } else if x > 400f32 - 64f32 && x < 400f32 + 64f32 {
                self.player.borrow_mut().color = self.color_green;
                self.finished = true;
            } else if x > 400f32 + 64f32 && x < 400f32 + 128f32 + 64f32 {
                self.player.borrow_mut().color = self.color_blue;
                self.finished = true;
            }
        }
    }
}

impl Scene for GameStartScene {
    fn finished(&self) -> bool {
        self.finished && self.drawed_loading_text
    }
}
