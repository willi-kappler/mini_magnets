// Rust modules
use std::rc::Rc;
use std::fs;
use std::error;
use std::fmt;
use std::io::Error as StdIOError;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use serde_derive::{Serialize, Deserialize};
use serde_json::error::Error as JSONError;

// Local modules
use crate::game::{GameScreen};
use crate::menu::{BaseMenu};
use crate::text_fx::{Font};

#[derive(Serialize, Deserialize, Debug)]
pub struct HighScore {
    scores: Vec<(u32, String)>,
}

impl HighScore {
    fn new() -> HighScore {
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
        }
    }

    fn to_text(&self) -> Vec<String> {
        self.scores.iter().map(|(s, n)| format!("{} - {}", s, n)).collect::<Vec<String>>()
    }
}

pub struct HighScoreMenu {
    base: BaseMenu,
    high_score: HighScore,
    filepath: String,
}

impl HighScoreMenu {
    pub fn new() -> HighScoreMenu {
        let high_score = HighScore::new();

        HighScoreMenu {
            base: BaseMenu::new(400, 100, 30, "CREDITS".to_string(), high_score.to_text(), vec!["BACK".to_string()]),
            high_score,
            filepath: "assets/highscore.json".to_string(),
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
        self.base.set_text(self.high_score.to_text());
    }

    pub fn load(&mut self) -> Result<(), HighScoreError> {
        let data = fs::read_to_string(&self.filepath)
            .map_err(|e| HighScoreError::ReadError(e, self.filepath.clone()))?;
        self.high_score = serde_json::from_str(&data)?;
        self.base.set_text(self.high_score.to_text());

        Ok(())
    }

    pub fn save(&self) -> Result<(), HighScoreError> {
        let data = serde_json::to_string(&self.high_score)?;
        fs::write(&self.filepath, data)
            .map_err(|e| HighScoreError::WriteError(e, self.filepath.clone()))?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum HighScoreError {
    ReadError(StdIOError, String),
    WriteError(StdIOError, String),
    ParseError(JSONError),
}

impl fmt::Display for HighScoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HighScoreError::ReadError(ref e, ref path) => {
                write!(f, "IO error while reading the high score file: '{}', {}", path, e)
            },
            HighScoreError::WriteError(ref e, ref path) => {
                write!(f, "IO error while writing the high score file: '{}', {}", path, e)
            },
            HighScoreError::ParseError(ref e) => {
                write!(f, "Parse error while accessing the high score file: {}", e)
            },
        }
    }
}

impl error::Error for HighScoreError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            HighScoreError::ReadError(ref e, _) => {
                Some(e)
            },
            HighScoreError::WriteError(ref e, _) => {
                Some(e)
            },
            HighScoreError::ParseError(ref e) => {
                Some(e)
            },
        }
    }
}

impl From<JSONError> for HighScoreError {
    fn from(e: JSONError) -> HighScoreError {
        HighScoreError::ParseError(e)
    }
}
