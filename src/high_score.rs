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


#[derive(Debug)]
pub struct HighScore {
    scores: Vec<(u32, String)>,
    file: String,
}

impl HighScore {
    fn new(file: &str) -> HighScore {
        HighScore {
            scores: vec![
                (1000, "WILLI KAPPLER".to_string()),
                 (900, "WILLI KAPPLER".to_string()),
                 (800, "WILLI KAPPLER".to_string()),
                 (700, "WILLI KAPPLER".to_string()),
                 (600, "WILLI KAPPLER".to_string()),
                 (500, "WILLI KAPPLER".to_string()),
                 (400, "WILLI KAPPLER".to_string()),
                 (300, "WILLI KAPPLER".to_string()),
                 (200, "WILLI KAPPLER".to_string()),
                 (100, "WILLI KAPPLER".to_string()),
            ],
            file: file.to_string(),
        }
    }

    fn load(&mut self) {

    }

    fn save(&mut self) {

    }

    fn to_text(&self) -> Vec<String> {
        self.scores.iter().map(|(s, n)| format!("{} - {}", s, n)).collect::<Vec<String>>()
/*
        let mut result: Vec<&str> = Vec::new();

        result
*/
    }
}

pub struct HighScoreMenu {
    base: BaseMenu,
    high_score: HighScore,
}

impl HighScoreMenu {
    pub fn new() -> HighScoreMenu {
        let high_score = HighScore::new("assets/highscore.txt");

        HighScoreMenu {
            base: BaseMenu::new(400, 100, 30, "CREDITS".to_string(), high_score.to_text(), vec!["BACK".to_string()]),
            high_score,
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
        self.base.update()
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.base.draw(canvas)
    }

    pub fn set_font(&mut self, font: &Rc<Font>) {
        self.base.set_font(font);
    }
}
