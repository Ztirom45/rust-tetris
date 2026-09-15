use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::config::*;
use crate::game::*;
use rand::{thread_rng, Rng};
                              
#[derive(Copy,Clone)]
pub struct Block{
    pub block_positions:[FPos;4],
    pub pos:FPos,
    pub speed: f32,
    pub color: Color,
}

impl Block{
    pub fn new()->Block{
        Block{
            block_positions:[FPos{x:0.0,y:0.0};4],
            pos:FPos{x:0.0,y:0.0},
            speed:NORMAL_SPEED,
            color:Color::RGB(255, 0, 0),
        }
    }
    
    pub fn random_new()->Self{
        let mut rng = thread_rng();
        Self{
            block_positions:BLOCK_CONSTALATIONS[rng.gen_range(0..BLOCK_CONSTALATIONS_LEN)],
            pos:FPos{x:rng.gen_range(0..BLOCK_W as i32-4) as f32,y:0.0},
            speed:NORMAL_SPEED,
            color:BLOCK_COLORS[rng.gen_range(0..BLOCK_COLORS_LEN)],
        }
    }
    pub fn update(&mut self){
        self.pos.y +=self.speed;
    }

    pub fn rotate(){}
    pub fn check_collision(/*placed_blocks*/){}
}

