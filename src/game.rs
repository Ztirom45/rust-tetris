use sdl2::pixels::Color;

use crate::config::*;
use crate::block::*;

pub struct Game{
    pub current_block:MovebleTile,
    //pub keyboard_heandler,
    pub placed_blocks:[[Option<Color>;BLOCK_W];BLOCK_H],
}

impl Game{
    pub fn new()-> Self{
        Self{
            current_block:MovebleTile::random_new(),
            placed_blocks:[[None;BLOCK_W];BLOCK_H],

        }
    }
    pub fn update(&mut self){
        self.current_block.update(&mut self.placed_blocks);
    }
}
