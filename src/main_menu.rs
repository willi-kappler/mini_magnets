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
use crate::text_fx::{Font, StaticText};

pub struct MainMenu {
    base: BaseMenu,
    fps: StaticText,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            base: BaseMenu::new(400, 100, 30, "MAIN MENU", Vec::new(), vec![
                "START",
                "AUDIO OPTIONS",
                "GFX OPTIONS",
                "CONTROLS",
                "HIGH SCORE",
                "CREDITS",
                "EXIT",
            ]),
            fps: StaticText::new(0, 575, "FPS"),
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
        self.fps.set_text(&fps_string);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.base.draw(canvas);
        self.fps.draw(canvas);
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.base.set_font(Rc::clone(&font));
        self.fps.set_font(Rc::clone(&font));
    }
}
