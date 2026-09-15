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
    pub fn random_reset(&mut self){
        let mut rng = thread_rng();
        self.block_positions = BLOCK_CONSTALATIONS[rng.gen_range(0..BLOCK_CONSTALATIONS_LEN)];
        self.pos = FPos{x:rng.gen_range(0..BLOCK_W as i32-4) as f32,y:0.0};
        self.speed = NORMAL_SPEED;
        self.color = BLOCK_COLORS[rng.gen_range(0..BLOCK_COLORS_LEN)];

    }

    pub fn update(&mut self,placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H]){
        self.pos.y +=self.speed;
        if self.check_collision(placed_blocks){
            //place block
            for pos in self.block_positions{
                let real_pos_x = (pos.x+self.pos.x) as usize;
                let real_pos_y = (pos.y+self.pos.y-1.0) as usize;
                placed_blocks[real_pos_y][real_pos_x] = Some(self.color);
            }
            self.random_reset();
        }
    }

    pub fn rotate(){}
    pub fn check_collision(&mut self,placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H])->bool{ 
        for pos in self.block_positions{
            //cheak if any part of the block colides with a placed block
            //(game.placed_blocks[x][y]=true)
            let real_pos_x = (pos.x+self.pos.x) as usize;
            let real_pos_y = (pos.y+self.pos.y) as usize;
            
            if real_pos_y >= BLOCK_H{
                return true;
            }
            //else index is part of placed blocks and can be used to check if block colides with
            //placed blocks
            if placed_blocks[real_pos_y][real_pos_x] != None 
            {
                return true;
            }
        }
        false
    }
}

