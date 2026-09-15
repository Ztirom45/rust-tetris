extern crate sdl2;

use std::time::Duration;

use sdl2::pixels::Color;

mod config;
mod block;
mod game;
mod sdl_head;

use crate::config::*;
use crate::game::*;
use crate::sdl_head::*;


fn main()  -> Result<(), String> {
    //inititlizing SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rusty Tetris", SCREEN_W, SCREEN_H)
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


    let mut game:Game = Game::new();
    
    //run game
    'running:loop{
        if sdl_handle_events(&mut game, &mut event_pump) {
            break 'running;
        }
        game.update();
        sdl_draw_game(&game,&mut canvas);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
