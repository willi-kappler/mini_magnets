// Rust modules
use std::rc::Rc;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

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
    filename: String,
}

impl HighScore {
    fn new(filename: String) -> HighScore {
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
            filename,
        }
    }

    fn load(&mut self) {
        let input = File::open(&self.filename).unwrap();
        let buffered = BufReader::new(input);

        for (line, score) in buffered.lines().zip(self.scores.iter_mut()) {
            let line = line.unwrap();
            let values: Vec<&str> = line.split(',').collect();
            if values.len() == 2 {
                score.0 = values[0].parse::<u32>().unwrap();
                score.1 = values[1].trim().to_string();
            }
        }

        self.scores.sort_by(|item1, item2| item2.0.cmp(&item1.0));
    }

    fn save(&self) {
        let mut output = File::create(&self.filename).unwrap();

        for (s, n) in self.scores.iter() {
            write!(output, "{}, {}\n", s, n).unwrap();
        }
    }

    fn to_text(&self) -> Vec<String> {
        self.scores.iter().map(|(s, n)| format!("{} - {}", s, n)).collect::<Vec<String>>()
    }
}

pub struct HighScoreMenu {
    base: BaseMenu,
    high_score: HighScore,
}

impl HighScoreMenu {
    pub fn new() -> HighScoreMenu {
        let high_score = HighScore::new("assets/highscore.txt".to_string());

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

    pub fn load(&mut self) {
        self.high_score.load();
        self.base.set_text(self.high_score.to_text());
    }

    pub fn save(&self) {
        self.high_score.save();
    }
}
