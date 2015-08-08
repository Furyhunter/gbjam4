#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;

mod game;
mod input;
mod math;
mod gfx;

use game::Game;

use log::LogLevel;

fn main() {
    env_logger::init().unwrap();

    info!("Initializing game");
    let mut game = Game::new().unwrap();
    info!("Initialized");

    info!("Running");
    game.run();
    info!("Exiting");
}
