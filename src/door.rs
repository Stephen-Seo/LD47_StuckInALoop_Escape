use ggez::graphics::{self, DrawMode, DrawParam, Image, Mesh, Rect};
use ggez::{Context, GameResult};

const DEFAULT_RADIUS: f32 = 70f32;

pub struct Door {
    is_open: bool,
    x: f32,
    y: f32,
    id: usize,
    enter_radius: f32,
}

impl Door {
    pub fn new(is_open: bool, x: f32, y: f32, id: usize) -> Self {
        Door {
            is_open,
            x,
            y,
            id,
            enter_radius: DEFAULT_RADIUS,
        }
    }

    pub fn draw(&self, ctx: &mut Context, door_image: &Image) -> GameResult<()> {
        if self.is_open {
            let bg_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0f32, 0f32, 96f32, 160f32),
                graphics::Color::BLACK,
            )?;
            graphics::draw(ctx, &bg_mesh, DrawParam::new().dest([self.x, self.y]))?;
            graphics::draw(
                ctx,
                door_image,
                DrawParam::new()
                    .src(Rect::new(0f32, 0.8f32, 1f32, 0.2f32))
                    .dest([self.x, self.y]),
            )?;
        } else {
            graphics::draw(ctx, door_image, DrawParam::new().dest([self.x, self.y]))?;
        }
        Ok(())
    }

    pub fn get_open(&self) -> bool {
        self.is_open
    }

    pub fn set_open(&mut self, is_open: bool) {
        self.is_open = is_open;
    }

    pub fn toggle_open(&mut self) -> bool {
        self.is_open = !self.is_open;
        self.is_open
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn is_within_range(&self, x: f32, y: f32) -> bool {
        let a = (self.x + 96f32 / 2f32) - x;
        let b = (self.y + 80f32) - y;
        (a * a + b * b).sqrt() <= self.enter_radius
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_center_x(&self) -> f32 {
        self.x + 96f32 / 2f32
    }

    pub fn get_center_y(&self) -> f32 {
        self.y + 80f32
    }
}
