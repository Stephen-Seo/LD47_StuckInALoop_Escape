use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};

use crate::scenes::gamestart::GameStartScene;
use crate::subeventhandler::SubEventHandler;

pub struct Game {
    current_scene: Vec<Box<dyn SubEventHandler>>,
    state: GameState,
}

enum GameState {
    GameStart,
    Opening,
}

impl GameState {
    fn set_scene(&self, ctx: &mut Context, current_scene: &mut Vec<Box<dyn SubEventHandler>>) {
        current_scene.clear();
        match self {
            GameState::GameStart => {
                current_scene.push(GameStartScene::new_boxed(ctx));
            }
            GameState::Opening => {}
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
            current_scene: Vec::new(),
            state: GameState::GameStart,
        };

        game.state.set_scene(ctx, &mut game.current_scene);

        game
    }

    fn scene_next(&mut self) -> GameResult<()> {
        for scene in &mut self.current_scene {
            scene.next();
        }
        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.current_scene.is_empty() {
            let mut finished = true;
            for scene in &mut self.current_scene {
                scene.update(ctx)?;
                if !scene.finished() {
                    finished = false;
                }
            }
            if finished {
                self.state = self.state.get_next_state();
                self.state.set_scene(ctx, &mut self.current_scene);
            }
        } else {
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        for scene in &mut self.current_scene {
            scene.draw(ctx)?;
        }

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if !self.current_scene.is_empty() {
            self.scene_next().unwrap();
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if !self.current_scene.is_empty() {
            self.scene_next().unwrap();
        }
    }
}
