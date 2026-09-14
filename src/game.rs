use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::config::*;
use crate::block::*;

pub struct Game<'a>{
    pub current_block:Block,
    //pub keyboard_heandler,
    pub placed_blocks:[[bool;BLOCK_W];BLOCK_H],
    pub canvas:&'a mut Canvas<Window>,  
    pub event_pump:&'a mut EventPump,
}

impl<'a> Game<'a>{
    pub fn new(canvas:&'a mut Canvas<Window>,event_pump:&'a mut EventPump)-> Self{
        Self{
            current_block:Block::random_new(),
            placed_blocks:[[true;BLOCK_W];BLOCK_H],
            canvas:canvas,
            event_pump:event_pump,

        }
    }
    pub fn handle_keyboard_input(&mut self) -> bool{//returns, if the mainloop should be stoped
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => (),
            }
        }
        false
    }
    pub fn draw(&mut self){
        // clear screen
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.current_block.draw(&mut self.canvas);
        self.canvas.present();
    }
    pub fn update(&self){}
    pub fn run(&mut self){
        'running:loop{
            if self.handle_keyboard_input() {
                break 'running;
            }
            self.draw();
            self.update();
        }
    }
}
