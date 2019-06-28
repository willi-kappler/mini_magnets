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
            base: BaseMenu::new(400, 100, 30, "CREDITS", vec![
                "CODE: WILLI KAPPLER",
                "IDEA: WILLI KAPPLER",
                "LEVELS: WILLI KAPPLER",
                "GFX: WILLI KAPPLER",
                "SFX: WILLI KAPPLER",
                "MUSIC: WILLI KAPPLER",
            ], vec!["BACK"]),
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
