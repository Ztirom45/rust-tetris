use sdl2::{pixels::Color, rect::Point};

pub const SCREEN_W:u32 = 600;
pub const SCREEN_H:u32 = 1200;
pub const BLOCK_W:usize = 8;
pub const BLOCK_H:usize = 16;

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

impl Pos{
    pub fn new() -> Self{
        Self{x:0,y:0}
    }
}
impl From<Pos> for Point{
    fn from(pos: Pos) -> Self{
        Point::new(pos.x,pos.y)
    }
}



pub const BLOCK_CONSTALATIONS_LEN:usize = 6;
pub const BLOCK_CONSTALATIONS:[[Pos;4];BLOCK_CONSTALATIONS_LEN] = [
    // ##
    // ##
    [Pos{x:0,y:0},Pos{x:0,y:1},Pos{x:1,y:0},Pos{x:1,y:1}],
    
    // #####
    [Pos{x:0,y:0},Pos{x:1,y:0},Pos{x:2,y:0},Pos{x:3,y:0}],
    
    // #
    // ###
    [Pos{x:0,y:0},Pos{x:1,y:0},Pos{x:1,y:1},Pos{x:1,y:2}],

    //   #
    // ###
    [Pos{x:0,y:2},Pos{x:1,y:0},Pos{x:1,y:1},Pos{x:1,y:2}],
    
    //  ##
    // ##
    [Pos{x:0,y:1},Pos{x:0,y:2},Pos{x:1,y:0},Pos{x:1,y:1}],

    // ##
    //  ##
    [Pos{x:0,y:1},Pos{x:0,y:0},Pos{x:1,y:2},Pos{x:1,y:1}],
];

