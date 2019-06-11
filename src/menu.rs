// External modules
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game_state::{GameState};

pub fn process(game_state: &mut GameState, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} => {
                // User closed main window
                game_state.quit = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                // User pressed ESC in main menu
                game_state.quit = true;
            },
            _ => {}
        }
    }
}

pub fn update() {

}

pub fn draw(game_state: &GameState, canvas: &mut Canvas<Window>) {

}
