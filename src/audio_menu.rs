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

pub struct AudioMenu {
    base: BaseMenu,
}

impl AudioMenu {
    pub fn new() -> AudioMenu {
        AudioMenu {
            base: BaseMenu::new(400, 100, 30, "AUDIO OPTIONS".to_string(), Vec::new(),
                vec![
                    "SFX VOLUME:".to_string(),
                    "MUSIC VOLUME:".to_string(),
                    "BACK".to_string()
                ]),
        }
    }

    pub fn process(&mut self, event: &Event, game_screen: &mut GameScreen, settings: &mut GameSettings) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                match self.base.get_selected() {
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
                        let old_vol = settings.get_sound_vol();
                        settings.dec_sound_vol();
                        let new_vol = settings.get_sound_vol();
                        if old_vol != new_vol {
                            // TODO: Change SDL sound volume
                            self.update_settings(settings);
                        }
                    },
                    1 => {
                        let old_vol = settings.get_music_vol();
                        settings.dec_music_vol();
                        let new_vol = settings.get_music_vol();
                        if old_vol != new_vol {
                            // TODO: Change SDL music volume
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
                        let old_vol = settings.get_sound_vol();
                        settings.inc_sound_vol();
                        let new_vol = settings.get_sound_vol();
                        if old_vol != new_vol {
                            // TODO: Change SDL sound volume
                            self.update_settings(settings);
                        }
                    },
                    1 => {
                        let old_vol = settings.get_music_vol();
                        settings.inc_music_vol();
                        let new_vol = settings.get_music_vol();
                        if old_vol != new_vol {
                            // TODO: Change SDL music volume
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
        self.base.change_menu(0, format!("SFX VOLUME: {}", settings.get_sound_vol()));
        self.base.change_menu(1, format!("MUSIC VOLUME: {}", settings.get_music_vol()));
    }
}
