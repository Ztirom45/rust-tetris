use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::config::*;
use crate::game::*;
use rand::{thread_rng, Rng};
                              
#[derive(Copy,Clone)]
pub struct Block{
    pub block_positions:[Point;4],
    pub speed: i32,
    pub color: Color,
}

impl Block{
    pub fn new()->Block{
        Block{
            block_positions:[Point::new(0,0);4],
            speed:1,
            color:Color::RGB(255, 0, 0),
        }
    }

    pub fn random_new()->Self{
        let mut rng = thread_rng();
        Self{
            block_positions:BLOCK_CONSTALATIONS[rng.gen_range(0..BLOCK_CONSTALATIONS_LEN)],
            speed:1,
            color:BLOCK_COLORS[rng.gen_range(0..BLOCK_COLORS_LEN)],
        }
    }
    pub fn update(&mut self,game:Game){
        for mut block_pos in self.block_positions{
            block_pos.y -= 1;
        }
    }
    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(self.color);
        canvas.draw_points(self.block_positions.as_slice()).unwrap();
    }
    pub fn rotate(){}
    pub fn check_collision(/*placed_blocks*/){}
}

