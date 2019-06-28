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

pub struct CreditMenu {
    base: BaseMenu,
    title: WaveText,
    text: Vec<StaticText>,
    back: SelectableText,
}

impl CreditMenu {
    pub fn new() -> CreditMenu {
        let mut back = SelectableText::new(200, 350, 25, "BACK");
        back.set_active(true);

        CreditMenu {
            base: BaseMenu::new(1),
            title: WaveText::new(200, 100, 10.0, 0.1, 0.5, "CREDITS"),
            text: CreditMenu::create_text(200, 150, 30, vec![
                "CODE: WILLI KAPPLER",
                "IDEA: WILLI KAPPLER",
                "LEVELS: WILLI KAPPLER",
                "GFX: WILLI KAPPLER",
                "SFX: WILLI KAPPLER",
                "MUSIC: WILLI KAPPLER",
            ]),
            back,
        }
    }

    fn create_text(x: u32, y: u32, step: u32, text: Vec<&str>) -> Vec<StaticText> {
        let mut result: Vec<StaticText> = Vec::new();
        let mut y2 = y;

        for item in text {
            result.push(StaticText::new(x, y2, item));
            y2 += step;
        }

        result
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
        self.title.update();
        self.back.update();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.title.draw(canvas);
        self.back.draw(canvas);

        for item in self.text.iter() {
            item.draw(canvas);
        }
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.title.set_font(Rc::clone(&font));
        self.back.set_font(Rc::clone(&font));

        for item in self.text.iter_mut() {
            item.set_font(Rc::clone(&font));
        }
    }
}