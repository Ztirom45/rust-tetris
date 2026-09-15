use crate::config::*;
use crate::block::*;

pub struct Game{
    pub current_block:Block,
    //pub keyboard_heandler,
    pub placed_blocks:[[bool;BLOCK_W];BLOCK_H],
}

impl Game{
    pub fn new()-> Self{
        Self{
            current_block:Block::random_new(),
            placed_blocks:[[true;BLOCK_W];BLOCK_H],

        }
    }
    pub fn update(&self){}
}
