use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use crate::scenes::gamestart::GameStartScene;
use crate::scenes::Scene;

pub struct Game {
    current_scene: Box<dyn Scene>,
    state: GameState,
}

enum GameState {
    GameStart,
    Opening,
}

impl GameState {
    fn get_scene(&self, ctx: &mut Context) -> Box<dyn Scene> {
        match self {
            GameState::GameStart => GameStartScene::new_boxed(ctx),
            GameState::Opening => GameStartScene::new_boxed(ctx),
        }
    }

    fn get_next_state(&self) -> GameState {
        match self {
            GameState::GameStart => GameState::Opening,
            GameState::Opening => GameState::GameStart,
        }
    }
}

impl Game {
    pub fn new(ctx: &mut Context) -> Game {
        let mut game = Game {
            current_scene: GameStartScene::new_boxed(ctx),
            state: GameState::GameStart,
        };

        game
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.current_scene.update(ctx)?;
        if self.current_scene.finished() {
            self.state = self.state.get_next_state();
            self.current_scene = self.state.get_scene(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.current_scene.draw(ctx)?;

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }
}
