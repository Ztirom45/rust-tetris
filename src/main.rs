extern crate sdl2;

mod config;
mod block;
mod game;

use crate::game::*;


fn main() {
    let mut game:Game = Game::new();
    game.run();
}
