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
use crate::text_fx::{Font, StaticText, WaveText, SelectableText};

pub struct MainMenu {
    base: BaseMenu,
    title: WaveText,
    fps: StaticText,
    menu_items: Vec<SelectableText>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let menu_items = BaseMenu::create_texts(300, 150, 30, 25,
                vec!["START", "AUDIO OPTIONS", "GFX OPTIONS", "CONTROLS", "HIGH SCORE", "CREDITS", "EXIT"]);

        MainMenu {
            base: BaseMenu::new(menu_items.len()),
            title: WaveText::new(300, 100, 10.0, 0.1, 0.5, "MAIN MENU"),
            fps: StaticText::new(0, 575, "FPS"),
            menu_items,
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
                let prev_selected = self.base.get_selected();
                self.base.process(event);
                let new_selected = self.base.get_selected();

                if prev_selected != new_selected {
                    self.menu_items[prev_selected].set_active(false);
                    self.menu_items[new_selected].set_active(true);
                }
            }
        }
    }

    pub fn update(&mut self, fps: u32) {
        self.title.update();

        let fps_string = format!("FPS: {}", fps);
        self.fps.set_text(&fps_string);

        self.menu_items[self.base.get_selected()].update();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.title.draw(canvas);
        self.fps.draw(canvas);

        for item in self.menu_items.iter() {
            item.draw(canvas);
        }
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.title.set_font(Rc::clone(&font));
        self.fps.set_font(Rc::clone(&font));

        for item in self.menu_items.iter_mut() {
            item.set_font(Rc::clone(&font));
        }
    }
}
