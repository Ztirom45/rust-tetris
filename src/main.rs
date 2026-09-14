extern crate sdl2;

use sdl2::pixels::Color;

mod config;
mod block;
mod game;

use crate::config::*;
use crate::game::*;



fn main()  -> Result<(), String> {
    //inititlizing SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rusty SpaceGame", SCREEN_W, SCREEN_H)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(BLOCK_W as u32,BLOCK_H as u32).unwrap(); 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?; 


    let mut game:Game = Game::new(&mut canvas,&mut event_pump);
    game.run();
    Ok(())
}
