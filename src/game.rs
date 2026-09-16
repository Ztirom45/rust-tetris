use sdl2::pixels::Color;

use crate::config::*;
use crate::block::*;

pub struct Game{
    pub current_block:MovebleTile,
    //pub keyboard_heandler,
    pub placed_blocks:[[Option<Color>;BLOCK_W];BLOCK_H],
    pub score:usize,
}

impl Game{
    pub fn new()-> Self{
        Self{
            current_block:MovebleTile::random_new(),
            placed_blocks:[[None;BLOCK_W];BLOCK_H],
            score:0,
        }
    }
    // returns if the game should stop
    // decided in moveble tile (see block.rs)
    // true->stop; false->continue
    pub fn update(&mut self) -> bool{
        return self.current_block.update(&mut self.placed_blocks, &mut self.score);
    }
}
