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
use crate::text_fx::{Font};

pub struct CreditMenu {
    base: BaseMenu,
}

impl CreditMenu {
    pub fn new() -> CreditMenu {
        CreditMenu {
            base: BaseMenu::new(400, 100, 30, "CREDITS".to_string(), vec![
                "CODE: WILLI KAPPLER".to_string(),
                "IDEA: WILLI KAPPLER".to_string(),
                "LEVELS: WILLI KAPPLER".to_string(),
                "GFX: WILLI KAPPLER".to_string(),
                "SFX: WILLI KAPPLER".to_string(),
                "MUSIC: WILLI KAPPLER".to_string(),
            ], vec!["BACK".to_string()]),
        }
    }

    pub fn process(&mut self, event: &Event, game_screen: &mut GameScreen) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                game_screen.main_menu();
            },
            _ => {
                self.base.process(event);
            }
        }
    }

    pub fn update(&mut self) {
        self.base.update();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.base.draw(canvas);
    }

    pub fn set_font(&mut self, font: &Rc<Font>) {
        self.base.set_font(font);
    }
}
