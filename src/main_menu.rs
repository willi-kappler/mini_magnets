// Rust modules
use std::rc::Rc;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game::{GameScreen};
use crate::menu::{BaseMenu};
use crate::text_fx::{Font, StaticText, WaveHText, WaveVText};

pub struct MainMenu {
    base: BaseMenu,
    fps: StaticText,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            base: BaseMenu::new(400, 100, 30, "MAIN MENU".to_string(), Vec::new(), vec![
                "START".to_string(),
                "AUDIO OPTIONS".to_string(),
                "GFX OPTIONS".to_string(),
                "CONTROLS".to_string(),
                "HIGH SCORE".to_string(),
                "CREDITS".to_string(),
                "EXIT".to_string(),
            ]),
            fps: StaticText::new(0, 575, "FPS".to_string()),
        }
    }

    pub fn process(&mut self, event: &Event, quit: &mut bool, game_screen: &mut GameScreen) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                match self.base.get_selected() {
                    0 => {
                        game_screen.start_game();
                    },
                    1 => {
                        game_screen.audio_options();
                    },
                    2 => {
                        game_screen.gfx_options();
                    },
                    3 => {
                        game_screen.controls();
                    },
                    4 => {
                        game_screen.high_score();
                    },
                    5 => {
                        game_screen.credit();
                    }
                    6 => {
                        *quit = true;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            },
            _ => {
                self.base.process(event);
            }
        }
    }

    pub fn update(&mut self, fps: u32) {
        self.base.update();
        let fps_string = format!("FPS: {}", fps);
        self.fps.set_text(fps_string);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.base.draw(canvas);
        self.fps.draw(canvas);
    }

    pub fn set_font(&mut self, font: &Rc<Font>) {
        self.base.set_font(font);
        self.fps.set_font(font);
    }
}
