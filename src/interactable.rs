use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};

const DEFAULT_RADIUS: f32 = 70f32;

#[derive(Copy, Clone, PartialEq)]
pub enum InteractableType {
    Door(usize),
}

pub struct Interactable {
    itype: InteractableType,
    x: f32,
    y: f32,
    radius: f32,
}

impl Interactable {
    pub fn new(itype: InteractableType, x: f32, y: f32) -> Self {
        Self {
            itype,
            x,
            y,
            radius: DEFAULT_RADIUS,
        }
    }

    pub fn is_within_range(&self, x: f32, y: f32) -> bool {
        let a = self.x - x;
        let b = self.y - y;
        (a * a + b * b).sqrt() <= self.radius
    }

    pub fn get_type(&self) -> InteractableType {
        self.itype
    }

    pub fn get_radius(&mut self) -> f32 {
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        match self.itype {
            InteractableType::Door(_) => {
                let panel_mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(0f32, 0f32, 14f32, 16f32),
                    Color::from_rgb(0x16, 0x9c, 0xd8),
                )?;
                graphics::draw(
                    ctx,
                    &panel_mesh,
                    DrawParam::new().dest([self.x - 7f32, self.y - 8f32]),
                )?;
            }
        }

        Ok(())
    }
}
