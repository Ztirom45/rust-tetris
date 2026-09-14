use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;
#[cfg(not(feature = "headless"))]
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::config::*;
use crate::block::*;

pub struct Game{
    pub current_block:Block,
    //pub keyboard_heandler,
    pub placed_blocks:[[bool;BLOCK_W];BLOCK_H],
    
    #[cfg(not(feature = "headless"))]
    pub canvas:Option<Canvas<Window>>,
        
    #[cfg(not(feature = "headless"))]
    pub event_pump:Option<EventPump>,
}

impl Game{
    pub fn new()-> Self{
        Self{
            current_block:Block::new(),
            placed_blocks:[[true;BLOCK_W];BLOCK_H],

            #[cfg(not(feature = "headless"))]
            canvas:None,
            
            #[cfg(not(feature = "headless"))]
            event_pump:None,

        }
    }

    pub fn init(&mut self){
        #[cfg(not(feature = "headless"))]
        {
        //inititlizing SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rusty Tetris", SCREEN_W, SCREEN_H)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();
        
        
        self.canvas = Some(window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string()).unwrap());
        

        
        match self.canvas.as_mut() {
            Some(canvas) => 
                {

                    canvas.set_logical_size(BLOCK_W as u32, BLOCK_H as u32);
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.clear();
                    canvas.present();
                    
                },
            None => (),
        };






        /*
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();*/
        self.event_pump = Some(sdl_context.event_pump().unwrap()); 
        }
    }
    pub fn handle_keyboard_input(&self) -> bool{//returns, if the mainloop should be stoped
        let Some(event_pump_obj) = self.event_pump;
        for event in self.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
            }
        }
        false
    }
    pub fn draw(&self){
        
    }
    pub fn update(&self){}
    pub fn run(&mut self){
        self.init();
        'running:loop{
            #[cfg(not(feature = "headless"))]
            {
            self.draw();
            }
            self.update();
        }
    }
}
