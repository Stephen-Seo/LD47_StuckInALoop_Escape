pub mod gamestart;

use ggez::event::EventHandler;

pub trait Scene: EventHandler {
    fn finished(&self) -> bool;
}

pub trait SubEventHandler: Scene {
    fn next(&mut self);
}
