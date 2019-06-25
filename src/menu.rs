// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game::{Game};
use crate::draw_text::{draw_text, draw_text_centered, Font};

#[derive(Debug)]
pub struct MenuState {
    menu_screen: MenuScreen,
    selected_item: u8,
}

#[derive(Debug)]
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

pub fn process(game_state: &mut Game) {
    for event in game_state.event_pump.poll_iter() {
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

pub fn update(game_state: &mut Game) {

}

pub fn draw(game_state: &mut Game) {
    let font = &game_state.fonts[0];
    let mut canvas = &mut game_state.canvas;

    draw_text(&mut canvas, font, 100, 50, "HELLO WORLD");
    let fps_string = format!("FPS: {:2.2}", 1000.0 / (game_state.elapsed as f64));
    draw_text(&mut canvas, font, 100, 100, &fps_string);

    let menu_screen = &game_state.menu_state.menu_screen;
    let selected_item = game_state.menu_state.selected_item;

    match menu_screen {
        MenuScreen::MainMenu => {
            let menu = vec!["MAIN MENU", "START", "AUDIO OPTIONS", "GFX OPTIONS", "HIGH SCORE", "EXIT"];
            draw_menu(&mut canvas, font, 400, 150, 50, menu);
        },
        MenuScreen::AudioMenu => {
            let menu = vec!["AUDIO MENU", "SFX VOLUME", "GFX VOLUME", "BACK"];
            draw_menu(&mut canvas, font, 400, 150, 50, menu);
        },
        MenuScreen::GraphicMenu => {
            let menu = vec!["GFX MENU", "SCREEN SIZE", "GAMMA CORRECTION", "BACK"];
            draw_menu(&mut canvas, font, 400, 150, 50, menu);
        },
        MenuScreen::ControllerMenu => {
            let menu = vec!["CONTROLLER MENU", "UP", "DOWN", "LEFT", "RIGHT", "BACK"];
            draw_menu(&mut canvas, font, 400, 150, 50, menu);
        },
        MenuScreen::HighScore => {

        },
    }
}

fn draw_menu(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, step: u32, texts: Vec<&str>) {
    let mut py = y;
    for line in texts {
        draw_text_centered(canvas, font, x, py, line);
        py += step;
    }
}
