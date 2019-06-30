// Rust modules
use std::error;
use std::fmt;
use std::io::Error as StdIOError;

// External modules
use serde_derive::{Serialize, Deserialize};
use serde_json::error::Error as JSONError;


#[derive(Debug)]
pub struct GameSettings {
    start_level: u8,
    sound_volume: u8,
    music_volume: u8,
    fullscreen: bool,
    resolution: ScreenResolution,
    filename: String,
}

impl GameSettings {
    pub fn new() -> GameSettings {
        GameSettings {
            start_level: 0,
            sound_volume: 255,
            music_volume: 255,
            fullscreen: false,
            resolution: ScreenResolution::R800_600,
            filename: "assets/settings.txt".to_string(),
        }
    }

    pub fn load(&mut self) -> Result<(), SettingsError>{
        Ok(())
    }

    pub fn save(&self) -> Result <(), SettingsError> {
        Ok(())
    }

    pub fn inc_sound_vol(&mut self) {
        if self.sound_volume < 255 {
            self.sound_volume += 1;
        }
    }

    pub fn dec_sound_vol(&mut self) {
        if self.sound_volume > 0 {
            self.sound_volume -= 1;
        }
    }

    pub fn inc_music_vol(&mut self) {
        if self.music_volume < 255 {
            self.music_volume += 1;
        }
    }

    pub fn dec_music_vol(&mut self) {
        if self.music_volume > 0 {
            self.music_volume -= 1;
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }
}

#[derive(Debug)]
enum ScreenResolution {
    R800_600,
    R1024_768,
    R1280_1024,
    R1900_1200,
}

#[derive(Debug)]
pub enum SettingsError {
    IOError(StdIOError),
    ParseError(JSONError),
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SettingsError::IOError(ref e) => {
                write!(f, "IO error while accessing the high score file: {}", e)
            },
            SettingsError::ParseError(ref e) => {
                write!(f, "Parse error while accessing the high score file: {}", e)
            },
        }
    }
}

impl error::Error for SettingsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SettingsError::IOError(ref e) => {
                Some(e)
            },
            SettingsError::ParseError(ref e) => {
                Some(e)
            },
        }
    }
}

impl From<StdIOError> for SettingsError {
    fn from(e: StdIOError) -> SettingsError {
        SettingsError::IOError(e)
    }
}

impl From<JSONError> for SettingsError {
    fn from(e: JSONError) -> SettingsError {
        SettingsError::ParseError(e)
    }
}
