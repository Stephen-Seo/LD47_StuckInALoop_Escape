use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use ggez::timer::delta;
use ggez::{Context, GameResult};

const WALK_TIME: f32 = 0.4f32;

pub enum PlayerState {
    Standing,
    Walking(bool, f32),
}

pub struct Player {
    sprite: Image,
    pub x: f32,
    pub y: f32,
    state: PlayerState,
    color: Color,
}

impl Player {
    pub fn new(ctx: &mut Context, color: Color) -> GameResult<Self> {
        Ok(Self {
            sprite: Image::new(ctx, "res/player_sprite.png")?,
            x: 0f32,
            y: 0f32,
            state: PlayerState::Standing,
            color,
        })
    }

    pub fn set_walking(&mut self, is_walking: bool) {
        if is_walking {
            self.state = PlayerState::Standing;
        } else {
            self.state = PlayerState::Walking(true, 0f32);
        }
    }
}

impl EventHandler for Player {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx);
        match &mut self.state {
            PlayerState::Standing => (),
            PlayerState::Walking(left, timer) => {
                *timer += dt.as_secs_f32();
                if *timer >= WALK_TIME {
                    *timer -= WALK_TIME;
                    *left = !*left;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match &self.state {
            PlayerState::Standing => {
                graphics::draw(
                    ctx,
                    &self.sprite,
                    DrawParam::new()
                        .src(Rect::new(0f32, 0f32, 0.3333333333333f32, 1f32))
                        .dest([self.x, self.y])
                        .color(self.color),
                )?;
            }
            PlayerState::Walking(left, _) => {
                if *left {
                    graphics::draw(
                        ctx,
                        &self.sprite,
                        DrawParam::new()
                            .src(Rect::new(
                                0.3333333333333f32,
                                0f32,
                                0.3333333333333f32,
                                1f32,
                            ))
                            .dest([self.x, self.y])
                            .color(self.color),
                    )?;
                } else {
                    graphics::draw(
                        ctx,
                        &self.sprite,
                        DrawParam::new()
                            .src(Rect::new(
                                0.6666666666666f32,
                                0f32,
                                0.3333333333333f32,
                                1f32,
                            ))
                            .dest([self.x, self.y])
                            .color(self.color),
                    )?;
                }
            }
        }
        Ok(())
    }
}
