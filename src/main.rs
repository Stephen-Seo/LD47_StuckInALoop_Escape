mod game;
mod player;
mod scenes;

use ggez::conf::WindowSetup;
use ggez::{event, ContextBuilder};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("ld47_stuckinaloop", "Stephen Seo")
        .window_setup(WindowSetup::default().vsync(true))
        .build()
        .unwrap();

    let mut game = game::Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("ERROR: {}", e),
    }
}
