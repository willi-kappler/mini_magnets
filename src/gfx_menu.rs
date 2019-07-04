// Rust modules
use std::rc::Rc;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game::{GameScreen};
use crate::settings::{GameSettings};
use crate::menu::{BaseMenu};
use crate::text_fx::{Font};

pub struct GFXMenu {
    base: BaseMenu,
}

impl GFXMenu {
    pub fn new() -> GFXMenu {
        GFXMenu {
            base: BaseMenu::new(400, 100, 30, "GFX OPTIONS".to_string(), Vec::new(),
                vec![
                    "FULLSCREEN:".to_string(),
                    "RESOLUTION:".to_string(),
                    "BACK".to_string()
                ]),
        }
    }

    pub fn process(&mut self, event: &Event, game_screen: &mut GameScreen, settings: &mut GameSettings) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                match self.base.get_selected() {
                    0 => {
                        settings.toggle_fullscreen();
                        // TODO: Change SDL fullscreen
                        self.update_settings(settings);
                    },
                    2 => {
                        game_screen.main_menu();
                    },
                    _ => {
                    }
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                match self.base.get_selected() {
                    0 => {
                        settings.toggle_fullscreen();
                        // TODO: Change SDL fullscreen
                        self.update_settings(settings);
                    },
                    1 => {
                        let old_res = settings.get_resolution();
                        settings.dec_resolution();
                        let new_res = settings.get_resolution();
                        if old_res != new_res {
                            // TODO: Change SDL resolution
                            self.update_settings(settings);
                        }
                    },
                    _ => {
                    }
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                match self.base.get_selected() {
                    0 => {
                        settings.toggle_fullscreen();
                        // TODO: Change SDL fullscreen
                        self.update_settings(settings);
                    },
                    1 => {
                        let old_res = settings.get_resolution();
                        settings.inc_resolution();
                        let new_res = settings.get_resolution();
                        if old_res != new_res {
                            // TODO: Change SDL resolution
                            self.update_settings(settings);
                        }
                    },
                    _ => {
                    }
                }
            },
            _ => {
                self.base.process(event);
            }
        }
    }

    pub fn update(&mut self) {
        self.base.update()
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.base.draw(canvas)
    }

    pub fn set_font(&mut self, font: &Rc<Font>) {
        self.base.set_font(font);
    }

    pub fn update_settings(&mut self, settings: &GameSettings) {
        self.base.change_menu(0, format!("FULLSCREEN: {}", if settings.get_fullscreen() { "ON" } else { "OFF" }));
        self.base.change_menu(1, format!("RESOLUTION: {}", settings.resolution_to_text()));
    }
}
