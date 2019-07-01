// Rust modules
// Rust modules
use std::fs;
use std::error;
use std::fmt;
use std::io::Error as StdIOError;

// External modules
use serde_derive::{Serialize, Deserialize};
use serde_json::error::Error as JSONError;


#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    start_level: u8,
    sound_volume: u8,
    music_volume: u8,
    fullscreen: bool,
    resolution: u8,
    filepath: String,
}

impl GameSettings {
    pub fn new() -> GameSettings {
        GameSettings {
            start_level: 0,
            sound_volume: 255,
            music_volume: 255,
            fullscreen: false,
            resolution: 0,
            filepath: "assets/settings.json".to_string(),
        }
    }

    pub fn load(&mut self) -> Result<(), SettingsError>{
        let data = fs::read_to_string(&self.filepath)
            .map_err(|e| SettingsError::ReadError(e, self.filepath.clone()))?;
        *self = serde_json::from_str(&data)?;

        Ok(())
    }

    pub fn save(&self) -> Result <(), SettingsError> {
        let data = serde_json::to_string(&self)?;
        fs::write(&self.filepath, data)
            .map_err(|e| SettingsError::WriteError(e, self.filepath.clone()))?;

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

    pub fn inc_resolution(&mut self) {
        if self.resolution < 3 {
            self.resolution += 1;
        }
    }

    pub fn dec_resolution(&mut self) {
        if self.resolution > 0 {
            self.resolution -= 1;
        }
    }

    pub fn resolution_to_text(&self) -> String {
        match self.resolution {
            0 => "800x600".to_string(),
            1 => "1024x768".to_string(),
            2 => "1280x1024".to_string(),
            3 => "1980x1280".to_string(),
            _ => {
                unimplemented!();
            }
        }
    }
}

#[derive(Debug)]
pub enum SettingsError {
    ReadError(StdIOError, String),
    WriteError(StdIOError, String),
    ParseError(JSONError),
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SettingsError::ReadError(ref e, ref path) => {
                write!(f, "IO error while reading the settings file: '{}', {}", path, e)
            },
            SettingsError::WriteError(ref e, ref path) => {
                write!(f, "IO error while writing the settings file: '{}', {}", path, e)
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
            SettingsError::ReadError(ref e, _) => {
                Some(e)
            },
            SettingsError::WriteError(ref e, _) => {
                Some(e)
            },
            SettingsError::ParseError(ref e) => {
                Some(e)
            },
        }
    }
}

impl From<JSONError> for SettingsError {
    fn from(e: JSONError) -> SettingsError {
        SettingsError::ParseError(e)
    }
}
