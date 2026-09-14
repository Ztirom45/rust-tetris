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


pub const BLOCK_CONSTALATIONS_LEN:usize = 6;
pub const BLOCK_CONSTALATIONS:[[Point;4];BLOCK_CONSTALATIONS_LEN] = [
    // ##
    // ##
    [Point::new(0,0),Point::new(0,1),Point::new(1,0),Point::new(1,0)],
    
    // #####
    [Point::new(0,0),Point::new(1,0),Point::new(2,0),Point::new(3,0)],
    
    // #
    // ###
    [Point::new(0,0),Point::new(1,0),Point::new(1,1),Point::new(1,2)],

    //   #
    // ###
    [Point::new(0,2),Point::new(1,0),Point::new(1,1),Point::new(1,2)],
    
    //  ##
    // ##
    [Point::new(0,1),Point::new(0,2),Point::new(1,0),Point::new(1,1)],

    // ##
    //  ##
    [Point::new(0,1),Point::new(0,0),Point::new(1,2),Point::new(1,1)],
];

