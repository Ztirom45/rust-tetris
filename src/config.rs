use std::ops::Add;

use sdl2::{pixels::Color, rect::Point};

pub const SCREEN_W:u32 = 500;
pub const SCREEN_H:u32 = 1200;
pub const BLOCK_W:usize = 10;
pub const BLOCK_H:usize = 24;

pub const NORMAL_SPEED:f32 = 0.2;
pub const SPEED_UP:f32 = 0.5;

pub const BLOCK_COLORS_LEN:usize = 5;
pub const BLOCK_COLORS:[Color;BLOCK_COLORS_LEN] = [
    Color::RGB(255, 0, 0),
    Color::RGB(0, 255, 0),
    Color::RGB(0, 0, 255,),
    Color::RGB(255, 255, 0),
    Color::RGB(255, 255, 0)
];

#[derive(Clone, Copy)]
pub struct Pos{//mutible point that can be parsed to a imutable sdl drawble point
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FPos{//mutible point that can be parsed to a imutable sdl drawble point
    pub x: f32,
    pub y: f32,
}

impl From<Pos> for Point{
    fn from(pos: Pos) -> Self{
        Point::new(pos.x,pos.y)
    }
}
impl From<FPos> for Point{
    fn from(pos: FPos) -> Self{
        Point::new(pos.x as i32,pos.y as i32)
    }
}

impl Add for FPos{ 
    type Output = Self;
    fn add(self,other:Self) -> Self{
        Self{
            x:self.x+other.x,
            y:self.y+other.y,
        }
    }
}



pub const BLOCK_CONSTALATIONS_LEN:usize = 6;
pub const BLOCK_CONSTALATIONS:[[FPos;4];BLOCK_CONSTALATIONS_LEN] = [
    // ##
    // ##
    [FPos{x:0.0,y:0.0},FPos{x:0.0,y:1.0},FPos{x:1.0,y:0.0},FPos{x:1.0,y:1.0}],
    
    // #####
    [FPos{x:0.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:2.0,y:0.0},FPos{x:3.0,y:0.0}],
    
    // #
    // ###
    [FPos{x:0.0,y:0.0},FPos{x:1.0,y:0.0},FPos{x:1.0,y:1.0},FPos{x:1.0,y:2.0}],

    //   #
    // ###
    [FPos{x:0.0,y:2.0},FPos{x:1.0,y:0.0},FPos{x:1.0,y:1.0},FPos{x:1.0,y:2.0}],
    
    //  ##
    // ##
    [FPos{x:0.0,y:1.0},FPos{x:0.0,y:2.0},FPos{x:1.0,y:0.0},FPos{x:1.0,y:1.0}],

    // ##
    //  ##
    [FPos{x:0.0,y:1.0},FPos{x:0.0,y:0.0},FPos{x:1.0,y:2.0},FPos{x:1.0,y:1.0}],
];

