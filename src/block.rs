use crate::config::*;
use crate::game::*;

#[derive(Copy,Clone)]
pub struct Block{
    pub block_positions:[Pos;4],
    pub speed: i32,
}

impl Block{
    pub fn new()->Block{
        Block{
            block_positions:[Pos::new();4],
            speed:1}
    }
    pub fn update(game:Game){
        
    }
    pub fn draw(){}
    pub fn rotate(){}
    pub fn check_collision(/*placed_blocks*/){}
}

