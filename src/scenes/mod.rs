pub mod gamestart;
pub mod mainscene;

use ggez::event::EventHandler;

pub trait Scene: EventHandler {
    fn finished(&self) -> bool;
}
