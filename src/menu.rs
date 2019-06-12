// External modules
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game_state::{GameState};
use crate::draw_text::{draw_text};

pub struct MenuState {
    menu_screen: MenuScreen,
    selected_item: u8,
}

pub enum MenuScreen {
    MainMenu,
    AudioMenu,
    GraphicMenu,
    ControllerMenu,
    HighScore,
}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {
            menu_screen: MenuScreen::MainMenu,
            selected_item: 0,
        }
    }
}

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
    draw_text(canvas, &game_state.fonts[0], 100, 100, "HELLO WORLD");

}
