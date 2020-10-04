use ggez::graphics::{self, Color, DrawMode, DrawParam, Font, Mesh, Rect, Scale, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::scenes::mainscene::PuzzleID;

const INFO_TEXT_POS: [f32; 2] = [400f32, 80f32];
const RESET_TEXT_POS: [f32; 2] = [100f32, 500f32];
const SKIP_TEXT_POS: [f32; 2] = [700f32, 500f32];

pub struct Puzzle {
    ptype: PuzzleID,
    tiles: Vec<bool>,
    key_pos: usize,
    key_pressed: bool,
    abort: bool,
    info_text: Text,
    reset_text: Text,
    skip_text: Text,
    force_solve: bool,
}

impl Puzzle {
    pub fn new(ptype: PuzzleID, font: Font) -> Self {
        let mut info_text = Text::new("Make all tiles green");
        info_text.set_font(font, Scale::uniform(30f32));
        let mut reset_text = Text::new("Reset");
        reset_text.set_font(font, Scale::uniform(20f32));
        let mut skip_text = Text::new("Skip");
        skip_text.set_font(font, Scale::uniform(20f32));

        let mut puzzle = Self {
            ptype,
            tiles: Vec::new(),
            key_pos: 0,
            key_pressed: true,
            abort: false,
            info_text,
            reset_text,
            skip_text,
            force_solve: false,
        };

        puzzle.reset();

        puzzle
    }

    pub fn reset(&mut self) {
        match self.ptype {
            PuzzleID::FarRightHall => {
                self.tiles.clear();

                self.tiles.push(true);
                self.tiles.push(false);
                self.tiles.push(true);

                self.tiles.push(false);
                self.tiles.push(false);
                self.tiles.push(false);

                self.tiles.push(false);
                self.tiles.push(false);
                self.tiles.push(true);
            }
            PuzzleID::Computer => {
                self.tiles.clear();

                self.tiles.push(false);
                self.tiles.push(false);
                self.tiles.push(false);

                self.tiles.push(true);
                self.tiles.push(false);
                self.tiles.push(false);

                self.tiles.push(false);
                self.tiles.push(false);
                self.tiles.push(false);
            }
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        match self.ptype {
            PuzzleID::FarRightHall => (),
            PuzzleID::Computer => (),
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            let bg_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(50f32, 50f32, 700f32, 500f32),
                Color::from_rgb(0x29, 0x8d, 0xff),
            )?;
            graphics::draw(ctx, &bg_mesh, DrawParam::new())?;
        }
        match self.ptype {
            PuzzleID::FarRightHall | PuzzleID::Computer => {
                let rect = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(0f32, 0f32, 90f32, 90f32),
                    graphics::WHITE,
                )?;
                for i in 0..9usize {
                    if self.tiles[i] {
                        graphics::draw(
                            ctx,
                            &rect,
                            DrawParam::new()
                                .dest([
                                    400f32 + (i % 3) as f32 * 100f32 - 150f32 + 5f32,
                                    300f32 + (i / 3) as f32 * 100f32 - 150f32 + 5f32,
                                ])
                                .color(Color::from_rgb(0, 0xff, 0)),
                        )?;
                    } else {
                        graphics::draw(
                            ctx,
                            &rect,
                            DrawParam::new()
                                .dest([
                                    400f32 + (i % 3) as f32 * 100f32 - 150f32 + 5f32,
                                    300f32 + (i / 3) as f32 * 100f32 - 150f32 + 5f32,
                                ])
                                .color(Color::from_rgb(0xff, 0, 0)),
                        )?;
                    }
                }
                if self.key_pressed {
                    let pointer = Mesh::from_triangles(
                        ctx,
                        &[[0f32, 0f32], [32f32, 0f32], [0f32, 32f32]],
                        graphics::WHITE,
                    )?;
                    graphics::draw(
                        ctx,
                        &pointer,
                        DrawParam::new().dest([
                            400f32 + (self.key_pos % 3) as f32 * 100f32 - 100f32,
                            300f32 + (self.key_pos / 3) as f32 * 100f32 - 100f32,
                        ]),
                    )?;
                }
            }
        }
        let info_text_width = self.info_text.width(ctx);
        graphics::draw(
            ctx,
            &self.info_text,
            DrawParam::new().dest([INFO_TEXT_POS[0] - info_text_width as f32, INFO_TEXT_POS[1]]),
        )?;
        graphics::draw(ctx, &self.reset_text, DrawParam::new().dest(RESET_TEXT_POS))?;
        graphics::draw(ctx, &self.skip_text, DrawParam::new().dest(SKIP_TEXT_POS))?;
        Ok(())
    }

    pub fn handle_click(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.key_pressed = false;
        match self.ptype {
            PuzzleID::FarRightHall | PuzzleID::Computer => {
                if y > 150f32 && y < 250f32 {
                    if x > 250f32 && x < 350f32 {
                        self.handle_puzzle_input(0);
                    } else if x > 350f32 && x < 450f32 {
                        self.handle_puzzle_input(1);
                    } else if x > 450f32 && x < 550f32 {
                        self.handle_puzzle_input(2);
                    }
                } else if y > 250f32 && y < 350f32 {
                    if x > 250f32 && x < 350f32 {
                        self.handle_puzzle_input(3);
                    } else if x > 350f32 && x < 450f32 {
                        self.handle_puzzle_input(4);
                    } else if x > 450f32 && x < 550f32 {
                        self.handle_puzzle_input(5);
                    }
                } else if y > 350f32 && y < 450f32 {
                    if x > 250f32 && x < 350f32 {
                        self.handle_puzzle_input(6);
                    } else if x > 350f32 && x < 450f32 {
                        self.handle_puzzle_input(7);
                    } else if x > 450f32 && x < 550f32 {
                        self.handle_puzzle_input(8);
                    }
                }
            }
        }
        let reset_width = self.reset_text.width(ctx);
        let skip_width = self.skip_text.width(ctx);
        if y > 490f32 && y < 530f32 {
            if x > 100f32 && x < 100f32 + reset_width as f32 {
                self.reset();
            } else if x > 700f32 && x < 700f32 + skip_width as f32 {
                self.force_solve = true;
            }
        }
    }

    pub fn handle_key(&mut self, _ctx: &mut Context, keycode: KeyCode) {
        match self.ptype {
            PuzzleID::FarRightHall | PuzzleID::Computer => {
                if keycode == KeyCode::A || keycode == KeyCode::Left {
                    if self.key_pos % 3 == 0 {
                        self.key_pos += 2;
                    } else {
                        self.key_pos -= 1;
                    }
                    self.key_pressed = true;
                } else if keycode == KeyCode::D || keycode == KeyCode::Right {
                    if self.key_pos % 3 == 2 {
                        self.key_pos -= 2;
                    } else {
                        self.key_pos += 1;
                    }
                    self.key_pressed = true;
                } else if keycode == KeyCode::W || keycode == KeyCode::Up {
                    if self.key_pos / 3 == 0 {
                        self.key_pos += 6;
                    } else {
                        self.key_pos -= 3;
                    }
                    self.key_pressed = true;
                } else if keycode == KeyCode::S || keycode == KeyCode::Down {
                    if self.key_pos / 3 == 2 {
                        self.key_pos -= 6;
                    } else {
                        self.key_pos += 3;
                    }
                    self.key_pressed = true;
                } else if keycode == KeyCode::E
                    || keycode == KeyCode::Space
                    || keycode == KeyCode::Return
                {
                    self.handle_puzzle_input(self.key_pos);
                    self.key_pressed = true;
                } else if keycode == KeyCode::Escape {
                    self.abort = true;
                }
            }
        }
    }

    fn handle_puzzle_input(&mut self, idx: usize) {
        match self.ptype {
            PuzzleID::FarRightHall => {
                self.tiles[idx] = !self.tiles[idx];
                if idx % 3 > 0 {
                    self.tiles[idx - 1] = !self.tiles[idx - 1];
                }
                if idx % 3 < 2 {
                    self.tiles[idx + 1] = !self.tiles[idx + 1];
                }
                if idx / 3 > 0 {
                    self.tiles[idx - 3] = !self.tiles[idx - 3];
                }
                if idx / 3 < 2 {
                    self.tiles[idx + 3] = !self.tiles[idx + 3];
                }
            }
            PuzzleID::Computer => {
                self.tiles[idx] = !self.tiles[idx];
                match idx {
                    0 | 2 | 6 | 8 => {
                        self.tiles[4] = !self.tiles[4];
                    }
                    1 | 5 => {
                        self.tiles[2] = !self.tiles[2];
                    }
                    3 | 7 => {
                        self.tiles[6] = !self.tiles[6];
                    }
                    4 => {
                        self.tiles[1] = !self.tiles[1];
                        self.tiles[7] = !self.tiles[7];
                    }
                    _ => unreachable!("There should only be 9 tiles"),
                }
            }
        }
    }

    pub fn is_solved(&self) -> bool {
        if self.force_solve {
            return true;
        }
        match self.ptype {
            PuzzleID::FarRightHall | PuzzleID::Computer => {
                let mut solved = true;
                for tile in &self.tiles {
                    if !tile {
                        solved = false;
                        break;
                    }
                }
                solved
            }
        }
    }

    pub fn is_abort(&self) -> bool {
        self.abort
    }
}
