pub const SCREEN_W:u32 = 600;
pub const SCREEN_H:u32 = 1200;
pub const BLOCK_W:usize = 8;
pub const BLOCK_H:usize = 16;

// some simple reuseble code peaces like structs 
#[derive(Copy,Clone)]
pub struct Pos{
    pub x:usize,
    pub y:usize,
}
impl Pos{
    pub fn new()->Self{
        Pos{x:0,y:0}
    }
}
