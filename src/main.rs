#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#![feature(unboxed_closures)]

#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate find_folder;
extern crate image;

mod game;
mod input;
mod math;
mod gfx;
mod assets;

use game::Game;

use log::LogLevel;

fn main() {
    env_logger::init().unwrap();

    info!("Initializing game");
    let mut game = Game::new().unwrap();
    info!("Initialized");

    info!("Running");
    match game.run() {
        Ok(_) => info!("Exited successfully"),
        Err(s) => error!("Exited abnormally: {}", s)
    }
}
