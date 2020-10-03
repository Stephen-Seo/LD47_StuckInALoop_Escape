use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Drawable, Image, Rect};
use ggez::timer::delta;
use ggez::{Context, GameResult};

const WALK_TIME: f32 = 0.34f32;

pub enum PlayerState {
    Standing,
    Walking(bool, f32),
}

pub struct Player {
    sprite: Image,
    pub x: f32,
    pub y: f32,
    pub rot: f32,
    state: PlayerState,
    pub color: Color,
    xflip: bool,
}

impl Player {
    pub fn new(ctx: &mut Context, color: Color) -> GameResult<Self> {
        Ok(Self {
            sprite: Image::new(ctx, "/player_sprite.png")?,
            x: 300f32,
            y: 300f32,
            rot: 0f32,
            state: PlayerState::Standing,
            color,
            xflip: false,
        })
    }

    pub fn set_walking(&mut self, is_walking: bool) {
        if is_walking {
            self.state = PlayerState::Standing;
        } else {
            self.state = PlayerState::Walking(true, 0f32);
        }
    }

    pub fn set_xflip(&mut self, xflip: bool) {
        self.xflip = xflip;
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
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

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match &self.state {
            PlayerState::Standing => {
                if self.xflip {
                    graphics::draw(
                        ctx,
                        &self.sprite,
                        DrawParam::new()
                            .src(Rect::new(0f32, 0f32, 0.3333333333333f32, 1f32))
                            .dest([self.x, self.y])
                            .rotation(self.rot)
                            .color(self.color)
                            .scale([-1f32, 1f32])
                            .offset([1f32, 0f32]),
                    )?;
                } else {
                    graphics::draw(
                        ctx,
                        &self.sprite,
                        DrawParam::new()
                            .src(Rect::new(0f32, 0f32, 0.3333333333333f32, 1f32))
                            .dest([self.x, self.y])
                            .rotation(self.rot)
                            .color(self.color),
                    )?;
                }
            }
            PlayerState::Walking(left, _) => {
                if *left {
                    if self.xflip {
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
                                .rotation(self.rot)
                                .color(self.color)
                                .scale([-1f32, 1f32])
                                .offset([1f32, 0f32]),
                        )?;
                    } else {
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
                                .rotation(self.rot)
                                .color(self.color),
                        )?;
                    }
                } else {
                    if self.xflip {
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
                                .rotation(self.rot)
                                .color(self.color)
                                .scale([-1f32, 1f32])
                                .offset([1f32, 0f32]),
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
                                .rotation(self.rot)
                                .color(self.color),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}
