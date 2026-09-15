use sdl2::{EventPump, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, sys::Window};

use crate::game::*;

pub fn sdl_draw_game(game:&Game,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        // clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        //draw blocks
        canvas.set_draw_color(game.current_block.color);
        canvas.draw_points(game.current_block.block_positions.map(|pos| (pos+game.current_block.pos).into()).as_slice()).unwrap();
        canvas.present();
}

pub fn sdl_handle_events(event_pump:&mut EventPump) -> bool{
    for event in event_pump.poll_iter() {
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
