mod game;
mod scenes;
mod subeventhandler;

use ggez::event;
use ggez::ContextBuilder;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("ld47_stuckinaloop", "Stephen Seo")
        .build()
        .unwrap();

    let mut game = game::Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("ERROR: {}", e),
    }
}
