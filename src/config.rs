use std::ops::Add;

use sdl2::{pixels::Color, rect::Point};

pub const SCREEN_W:u32 = 400;
pub const SCREEN_H:u32 = 960;
pub const BLOCK_W:usize = 10;
pub const BLOCK_H:usize = 24;

pub const SPEED_INCREES_PER_TICK:f32 = 0.001;
pub const NORMAL_SPEED:f32 = 0.05;
pub const SPEED_UP_FAKTOR:f32 = 5.0;

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

