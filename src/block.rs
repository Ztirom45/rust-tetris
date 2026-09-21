use std::iter::zip;

use sdl2::pixels::Color;
use crate::config::*;
use rand::{thread_rng, Rng};


pub struct Tile{
    pub block_positions:[FPos;4],
    pub width:f32,//only natural numbers or zero in float notation
    pub height:f32,//only natural numbers or zero in float notation
}

pub const BLOCK_CONSTALATIONS_LEN:usize = 7;
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
    Tile{block_positions:[FPos{x:2.0,y:0.0},FPos{x:0.0,y:1.0},FPos{x:1.0,y:1.0},FPos{x:2.0,y:1.0}],width:3.0,height:2.0},
    
    //  ##
    // ##
    Tile{block_positions:[FPos{x:0.0,y:1.0},FPos{x:1.0,y:1.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:0.0}],width:3.0,height:2.0},

    // ##
    //  ##
    Tile{block_positions:[FPos{x:0.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:1.0},FPos{x:1.0,y:1.0}],width:3.0,height:2.0},

    //###
    // #
    Tile{block_positions:[FPos{x:0.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:0.0},FPos{x:1.0,y:1.0}],width:3.0,height:2.0}

];



#[derive(Copy,Clone)]
pub struct MovebleTile{
    pub block_positions:[FPos;4],
    pub width:f32,
    pub height:f32,
    pub pos:FPos,
    pub speed: f32,
    pub color: Color,
    pub speed_up: bool,
}

impl MovebleTile{
    pub fn random_new()->Self{
        let mut rng = thread_rng();
        let sub_tile = &BLOCK_CONSTALATIONS[rng.gen_range(0..BLOCK_CONSTALATIONS_LEN)];
        Self{
            block_positions:sub_tile.block_positions,
            height:sub_tile.height,
            width:sub_tile.width,
            pos:FPos{x:rng.gen_range(0..BLOCK_W as i32-4) as f32,y:0.0},
            speed:NORMAL_SPEED,
            speed_up:false,
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
        self.speed_up = false;
        self.color = BLOCK_COLORS[rng.gen_range(0..BLOCK_COLORS_LEN)];

    }
    
    pub fn get_speed(&mut self) -> f32{
        if self.speed_up{
            return (self.speed*SPEED_UP_FAKTOR).min(MAX_SPEED);
        }
        self.speed.min(MAX_SPEED)
    }

    //returns if the game should continue
    // false -> continue; true -> stop
    pub fn update(&mut self,placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H], score:&mut usize) -> bool{
        //handle movement
        self.pos.y += self.get_speed();

        if self.check_collision(placed_blocks){
            //place block
            for pos in self.block_positions{
                let real_pos_x = pos.x+self.pos.x;
                let real_pos_y = pos.y+self.pos.y-1.0;
                
                //cheak if a block is place above the upper border
                //if so: stop game
                if real_pos_y < 0.0{
                    return true;
                }

                placed_blocks[real_pos_y as usize][real_pos_x as usize] = Some(self.color);
            }
            self.remove_complete_rows(placed_blocks,score);

            self.random_reset();
        }
        false
    }

    pub fn remove_complete_rows(&mut self,placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H],score:&mut usize){
        //check for complete rows
        let mut number_completed_rows:usize = 0;
        let mut completed_rows:Vec<usize> = vec![];
        for y in (self.pos.y-1.0) as usize..(((self.pos.y+self.height+1.0)) as usize).min(BLOCK_H){
            let mut row_is_completed = true;
            for x in 0..BLOCK_W{
                if placed_blocks[y][x] == None{
                    row_is_completed = false;
                    break;
                }
            }
            if row_is_completed{
                number_completed_rows +=1;
                completed_rows.push(y);
            }
        }
        completed_rows.sort();
        completed_rows.reverse();
        //remove completed rows by moving the rows above down
        for i in 0..number_completed_rows{
            //do for every complerted_row[i+1]..completed_row[i] intervall until i =
            //number_completed_rows-2
            //last interval number_of_completed_rows..completed_row[i]
            let interval;
            if i < number_completed_rows-1{//every time except for the last time
                interval = completed_rows[i+1]..completed_rows[i];
            }else{//last interval
                interval = number_completed_rows-1..completed_rows[i];
            }
            
            for y in interval.rev(){
                placed_blocks[y+i+1] = placed_blocks[y];
            }
        }
        //fill new empty top rows
        for y in 0..number_completed_rows{
            *score += 1;
            self.speed+=SPEED_INCREES_PER_FULL_ROW;
            println!("{}",self.speed);
            placed_blocks[y] = [None;BLOCK_W]; 
        }
       
    }

    pub fn move_right(&mut self,placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H]){
        if ((self.pos.x+self.width) as i32) < (BLOCK_W as i32){
                self.pos.x += 1.0;
                //collision with other blocks check
                //check collision for next frame,
                //otherwise moving parts up, by timing side moves would be posible
                self.pos.y+=self.get_speed();
                if self.check_collision(placed_blocks){
                    self.pos.x-=1.0;
                }
                self.pos.y-=self.get_speed();
        }

    }
 
    pub fn move_left(&mut self,placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H]){
        if self.pos.x > 0.0{
                self.pos.x -= 1.0;
                //collision with other blocks check
                //check collision for next frame,
                //otherwise moving parts up, by timing side moves would be posible
                self.pos.y+=self.get_speed();

                if self.check_collision(placed_blocks){
                    self.pos.x+=1.0;
                }
                self.pos.y-=self.get_speed();

        }

    }

    pub fn rotate_right(&mut self, placed_blocks:&mut [[Option<Color>;BLOCK_W];BLOCK_H]){
        //check if rotating, which means swaping width and height would let a part leave the
        //game border
        //TODO: collision with other parts 
        if ((self.pos.x+self.height) as i32) > (BLOCK_W as i32){
            return;
        }

        let old_height = self.height;
        let old_width = self.width;
        let old_block_positions:[FPos;4] = self.block_positions;
        for (new_pos,old_pos) in zip(self.block_positions.as_mut(),old_block_positions){
            new_pos.y = old_pos.x;
            new_pos.x = old_height-1.0-old_pos.y;
        }
        //check if the rotated block colides with existing blocks
        if self.check_collision(placed_blocks){
            self.block_positions = old_block_positions;
            return;
        }
        //else swap width and height
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

