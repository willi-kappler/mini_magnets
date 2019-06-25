// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game::{Game};
use crate::game_screen::{GameScreen};
use crate::draw_text::{draw_text, draw_text_centered, Font};

#[derive(Debug)]
pub struct MainMenu {
    selected_item: usize,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            selected_item: 0,
        }
    }
}

impl GameScreen for MainMenu {
    fn process(&mut self, game: &mut Game) {
        for event in game.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    // User closed main window
                    game.quit = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // User pressed ESC in main menu
                    game.quit = true;
                },
                _ => {}
            }
        }
    }

    fn update(&mut self, game: &mut Game) {

    }

    fn draw(&mut self, game: &mut Game) {
        let font = &game.fonts[0];
        let mut canvas = &mut game.canvas;

        draw_text(&mut canvas, font, 100, 50, "HELLO WORLD");
        let fps_string = format!("FPS: {:2.2}", 1000.0 / (game.elapsed as f64));
        draw_text(&mut canvas, font, 100, 75, &fps_string);
        let elapsed_string = format!("ELAPSED: {}", game.elapsed);
        draw_text(&mut canvas, font, 100, 100, &elapsed_string);

        let menu = vec!["MAIN MENU", "START", "AUDIO OPTIONS", "GFX OPTIONS", "HIGH SCORE", "EXIT"];
        draw_menu(&mut canvas, font, 400, 125, 25, menu);
    }
}

fn draw_menu(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, step: u32, texts: Vec<&str>) {
    let mut py = y;
    for line in texts {
        draw_text_centered(canvas, font, x, py, line);
        py += step;
    }
}
