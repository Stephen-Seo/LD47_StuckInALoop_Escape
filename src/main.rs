mod door;
mod game;
mod interactable;
mod player;
mod puzzle;
mod scenes;

use ggez::conf::WindowSetup;
use ggez::{event, ContextBuilder};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("ld47_stuckinaloop", "Stephen Seo")
        .window_setup(
            WindowSetup::default()
                .title("LudumDare 47 - Stuck In A Loop - Escape")
                .vsync(true),
        )
        .build()
        .unwrap();

    let game = game::Game::new(&mut ctx);

    event::run(ctx, event_loop, game);
}
