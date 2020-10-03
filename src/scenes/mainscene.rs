use std::cell::RefCell;
use std::rc::Rc;

use ggez::audio::{SoundSource, Source};
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Image, Mesh, Rect, Scale, Text};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::timer::delta;
use ggez::{Context, GameResult};

use super::Scene;
use crate::player::Player;

const DARKNESS_PAN_RATE: f32 = 40f32;
const FLICKER_TIME: [f32; 6] = [1f32, 0.1f32, 0.85f32, 0.07f32, 0.12f32, 0.09f32];
const FLICKER_STATE: [bool; 6] = [true, false, true, false, true, false];
const TEXT_RATE: f32 = 0.3f32;
const TEXT_FAST_RATE: f32 = 0.1f32;
const IN_POD_TEXT_WAIT_TIME: f32 = 1f32;
const GET_OUT_OF_POD_TIME: f32 = 3f32;
const PLAYER_MOVEMENT_SPEED: f32 = 200f32;

enum State {
    InPod_InDarkness,
    InPod_WakeupText,
    GetOutOfPod,
    Investigate,
}

enum Room {
    StasisPod,
}

enum WalkingState {
    Standing,
    Left,
    Right,
}

pub struct MainScene {
    font: Font,
    player: Rc<RefCell<Player>>,
    finished: bool,
    current_text: Text,
    final_text: String,
    text_sfx: Source,
    music: Source,
    pod_image: Image,
    pod_flicker_image: Image,
    ground_rect: Rect,
    state: State,
    darkness_image: Image,
    darkness_yoffset: f32,
    timer: f32,
    draw_flicker_pod: bool,
    index: usize,
    room: Room,
    walking_state: WalkingState,
}

impl MainScene {
    pub fn new(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Self {
        let mut music = Source::new(ctx, "/music00.ogg").unwrap();
        music.set_repeat(true);
        //        music.play().unwrap();
        let mut current_text = Text::new("".to_owned());
        current_text.set_font(font, Scale::uniform(26f32));
        Self {
            font,
            player,
            finished: false,
            current_text,
            final_text: String::new(),
            text_sfx: Source::new(ctx, "/text.ogg").unwrap(),
            music,
            pod_image: Image::new(ctx, "/stasis_pod.png").unwrap(),
            pod_flicker_image: Image::new(ctx, "/stasis_pod_empty.png").unwrap(),
            ground_rect: Rect::new(0f32, 550f32, 800f32, 50f32),
            state: State::InPod_InDarkness,
            darkness_image: Image::new(ctx, "/darkness.png").unwrap(),
            darkness_yoffset: 0f32,
            timer: FLICKER_TIME[0],
            draw_flicker_pod: false,
            index: 0usize,
            room: Room::StasisPod,
            walking_state: WalkingState::Standing,
        }
    }

    pub fn new_boxed(ctx: &mut Context, font: Font, player: Rc<RefCell<Player>>) -> Box<Self> {
        Box::new(Self::new(ctx, font, player))
    }
}

impl EventHandler for MainScene {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx).as_secs_f32();
        match self.state {
            State::InPod_InDarkness => {
                let mut player = self.player.borrow_mut();
                player.x = 520f32;
                player.y = 350f32;
                player.rot = 0.78f32;
                self.timer -= dt;
                if self.timer <= 0f32 {
                    self.draw_flicker_pod = FLICKER_STATE[self.index];
                    self.index = (self.index + 1) % 6;
                    self.timer = FLICKER_TIME[self.index];
                }
                if self.darkness_yoffset > -300f32 {
                    self.darkness_yoffset -= dt * DARKNESS_PAN_RATE;
                } else {
                    self.state = State::InPod_WakeupText;
                    self.timer = TEXT_RATE;
                    self.final_text = "What.. Where am I?..".chars().rev().collect::<String>();
                }
            }
            State::InPod_WakeupText => {
                if !self.final_text.is_empty() {
                    self.timer -= dt;
                    if self.timer <= 0f32 {
                        self.timer = TEXT_RATE;
                        self.current_text.fragments_mut()[0]
                            .text
                            .push(self.final_text.pop().unwrap());
                        self.text_sfx.play()?;
                        if self.final_text.is_empty() {
                            self.timer = IN_POD_TEXT_WAIT_TIME;
                        }
                    }
                } else {
                    self.timer -= dt;
                    if self.timer <= 0f32 {
                        self.timer = 0f32;
                    }
                }
            }
            State::GetOutOfPod => {
                self.timer -= dt;
                if self.timer > 0f32 {
                    let mut player = self.player.borrow_mut();
                    let lerp = self.timer / GET_OUT_OF_POD_TIME;
                    player.x = (520f32 * lerp) + (500f32 * (1f32 - lerp));
                    player.y = (350f32 * lerp) + (430f32 * (1f32 - lerp));
                    player.rot = (0.78f32 * lerp) + (0f32 * (1f32 - lerp));
                } else if self.timer <= 0f32 {
                    self.state = State::Investigate;
                    self.music.play()?;
                }
            }
            State::Investigate => {
                let mut player = self.player.borrow_mut();
                match self.walking_state {
                    WalkingState::Standing => {
                        player.set_walking(false);
                    }
                    WalkingState::Left => {
                        player.x -= dt * PLAYER_MOVEMENT_SPEED;
                        if player.x <= 0f32 {
                            player.x = 0f32;
                            self.walking_state = WalkingState::Standing;
                            player.set_walking(false);
                        } else {
                            player.set_walking(true);
                            player.set_xflip(true);
                        }
                    }
                    WalkingState::Right => {
                        player.x += dt * PLAYER_MOVEMENT_SPEED;
                        if player.x + 64f32 >= 800f32 {
                            player.x = 800f32 - 64f32;
                            self.walking_state = WalkingState::Standing;
                            player.set_walking(false);
                        } else {
                            player.set_walking(true);
                            player.set_xflip(false);
                        }
                    }
                }
            }
        }
        self.player.borrow_mut().update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            let ground_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                self.ground_rect,
                Color::from_rgb(0x49, 0x49, 0x49),
            )?;
            graphics::draw(ctx, &ground_mesh, DrawParam::new())?;
        }

        match self.state {
            State::InPod_InDarkness => {
                if self.draw_flicker_pod {
                    graphics::draw(
                        ctx,
                        &self.pod_flicker_image,
                        DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                    )?;
                } else {
                    graphics::draw(
                        ctx,
                        &self.pod_image,
                        DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                    )?;
                }
                self.player.borrow_mut().draw(ctx)?;
            }
            State::InPod_WakeupText | State::GetOutOfPod => {
                graphics::draw(
                    ctx,
                    &self.pod_flicker_image,
                    DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                )?;
                self.player.borrow_mut().draw(ctx)?;
            }
            State::Investigate => {
                match self.room {
                    Room::StasisPod => {
                        graphics::draw(
                            ctx,
                            &self.pod_flicker_image,
                            DrawParam::new().dest([600f32, 170f32]).rotation(0.7f32),
                        )?;
                    }
                }
                self.player.borrow_mut().draw(ctx)?;
            }
        }

        graphics::draw(
            ctx,
            &self.darkness_image,
            DrawParam::new().dest([0f32, self.darkness_yoffset]),
        )?;

        match self.state {
            State::InPod_InDarkness => (),
            State::InPod_WakeupText => {
                graphics::draw(
                    ctx,
                    &self.current_text,
                    DrawParam::new().dest([100f32, 100f32]),
                )?;
            }
            State::GetOutOfPod => (),
            State::Investigate => (),
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match self.state {
            State::InPod_InDarkness => (),
            State::InPod_WakeupText => {
                if self.final_text.is_empty() && self.timer <= 0f32 {
                    self.state = State::GetOutOfPod;
                    self.timer = GET_OUT_OF_POD_TIME;
                } else {
                    self.timer = 0f32;
                }
            }
            State::GetOutOfPod => (),
            State::Investigate => {
                if button == MouseButton::Left {
                    let player = self.player.borrow();
                    if player.x > x {
                        self.walking_state = WalkingState::Left;
                    } else if player.x + 64f32 < x {
                        self.walking_state = WalkingState::Right;
                    }
                }
            }
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match self.state {
            State::InPod_InDarkness => (),
            State::InPod_WakeupText => (),
            State::GetOutOfPod => (),
            State::Investigate => {
                if button == MouseButton::Left {
                    self.walking_state = WalkingState::Standing;
                }
            }
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match self.state {
            State::InPod_InDarkness => (),
            State::InPod_WakeupText => {
                if self.final_text.is_empty() && self.timer <= 0f32 {
                    self.state = State::GetOutOfPod;
                    self.timer = GET_OUT_OF_POD_TIME;
                    self.current_text.fragments_mut()[0].text.clear();
                } else {
                    self.timer = 0f32;
                }
            }
            State::GetOutOfPod => (),
            State::Investigate => {
                if keycode == KeyCode::A || keycode == KeyCode::Left {
                    if self.player.borrow().x > 0f32 {
                        self.walking_state = WalkingState::Left;
                    }
                } else if keycode == KeyCode::D || keycode == KeyCode::Right {
                    if self.player.borrow().x + 64f32 < 800f32 {
                        self.walking_state = WalkingState::Right;
                    }
                }
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match self.state {
            State::InPod_InDarkness => (),
            State::InPod_WakeupText => (),
            State::GetOutOfPod => (),
            State::Investigate => {
                if keycode == KeyCode::A || keycode == KeyCode::Left {
                    self.walking_state = WalkingState::Standing;
                } else if keycode == KeyCode::D || keycode == KeyCode::Right {
                    self.walking_state = WalkingState::Standing;
                }
            }
        }
    }
}

impl Scene for MainScene {
    fn finished(&self) -> bool {
        self.finished
    }
}
