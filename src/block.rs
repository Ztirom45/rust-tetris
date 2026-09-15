use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::config::*;
use crate::game::*;
use rand::{thread_rng, Rng};


pub struct Tile{
    block_positions:[FPos;4],
    width:f32,//only natural numbers or zero in float notation
    height:f32,//only natural numbers or zero in float notation
}

pub const BLOCK_CONSTALATIONS_LEN:usize = 6;
pub const BLOCK_CONSTALATIONS:[Tile;BLOCK_CONSTALATIONS_LEN] = [
    // ##
    // ##
    Tile{block_positions:[FPos{x:0.0,y:0.0},FPos{x:0.0,y:1.0},FPos{x:1.0,y:0.0},FPos{x:1.0,y:1.0}],width:2.0,height:2.0},
    
    // #####
    Tile{block_positions:[FPos{x:0.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:0.0},FPos{x:3.0,y:0.0}],width:4.0,height:1.0},
    
    // #
    // ###
    Tile{block_positions:[FPos{x:0.0,y:0.0},FPos{x:0.0,y:1.0},FPos{x:1.0,y:1.0},FPos{x:2.0,y:1.0}],width:3.0,height:2.0},

    //   #
    // ###
    Tile{block_positions:[FPos{x:2.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:1.0,y:1.0},FPos{x:2.0,y:1.0}],width:3.0,height:2.0},
    
    //  ##
    // ##
    Tile{block_positions:[FPos{x:0.0,y:1.0},FPos{x:1.0,y:1.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:0.0}],width:3.0,height:2.0},

    // ##
    //  ##
    Tile{block_positions:[FPos{x:0.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:1.0},FPos{x:1.0,y:1.0}],width:3.0,height:2.0}
];



#[derive(Copy,Clone)]
pub struct MovebleTile{
    pub block_positions:[FPos;4],
    pub width:f32,
    pub height:f32,
    pub pos:FPos,
    pub speed: f32,
    pub color: Color,
}

impl MovebleTile{
    pub fn new()->MovebleTile{
        MovebleTile{
            block_positions:[FPos{x:0.0,y:0.0};4],
            width:0.0,
            height:0.0,
            pos:FPos{x:0.0,y:0.0},
            speed:NORMAL_SPEED,
            color:Color::RGB(255, 0, 0),
        }
    }
    
    pub fn random_new()->Self{
        let mut rng = thread_rng();
        let sub_tile = &BLOCK_CONSTALATIONS[rng.gen_range(0..BLOCK_CONSTALATIONS_LEN)];
        Self{
            block_positions:sub_tile.block_positions,
            height:sub_tile.height,
            width:sub_tile.width,
            pos:FPos{x:rng.gen_range(0..BLOCK_W as i32-4) as f32,y:0.0},
            speed:NORMAL_SPEED,
            color:BLOCK_COLORS[rng.gen_range(0..BLOCK_COLORS_LEN)],
        }
    }
    pub fn random_reset(&mut self){
        let mut rng = thread_rng();
        let sub_tile = &BLOCK_CONSTALATIONS[rng.gen_range(0..BLOCK_CONSTALATIONS_LEN)];
        self.block_positions = sub_tile.block_positions;
        self.width = sub_tile.width;
        self.height = sub_tile.height;
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
    
    pub fn move_right(&mut self){
        if ((self.pos.x+self.width) as i32) < (BLOCK_W as i32){
                self.pos.x += 1.0;
        }

    }
 
    pub fn move_left(&mut self){
        if self.pos.x > 0.0{
                self.pos.x -= 1.0;
        }

    }

    pub fn rotate_right(&mut self){
        //check if rotating, which means swaping width and height would let a part leave the
        //game border
        //TODO: collision with other parts 
        if ((self.pos.x+self.height) as i32) >= (BLOCK_W as i32){
            return;
        }

        let old_height = self.height;
        let old_width = self.width;
        for pos in self.block_positions.iter_mut(){
            let old_pos_x = pos.x;
            let old_pos_y = pos.y;
            pos.y = old_pos_x;
            pos.x = old_height-old_pos_y;
        }
        self.height = old_width;
        self.width = old_height;
    }
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

