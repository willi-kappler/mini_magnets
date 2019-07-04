// Rust modules
// Rust modules
use std::fs;
use std::error;
use std::fmt;
use std::io::Error as StdIOError;

// External modules
use serde_derive::{Serialize, Deserialize};
use serde_json::error::Error as JSONError;

const MAX_VOLUME: i16 = 255;
const MAX_RESOLUTION: i16 = 3;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    start_level: u8,
    sound_volume: i16,
    music_volume: i16,
    fullscreen: bool,
    resolution: i16,
    filepath: String,
}

impl GameSettings {
    pub fn new() -> GameSettings {
        GameSettings {
            start_level: 0,
            sound_volume: 200,
            music_volume: 200,
            fullscreen: false,
            resolution: 0,
            filepath: "assets/settings.json".to_string(),
        }
    }

    pub fn load(&mut self) -> Result<(), SettingsError> {
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
        self.sound_volume += 5;
        limit_range(&mut self.sound_volume, 0, MAX_VOLUME);
    }

    pub fn dec_sound_vol(&mut self) {
        self.sound_volume -= 5;
        limit_range(&mut self.sound_volume, 0, MAX_VOLUME);
    }

    pub fn get_sound_vol(&self) -> i16 {
        self.sound_volume
    }

    pub fn inc_music_vol(&mut self) {
        self.music_volume += 5;
        limit_range(&mut self.music_volume, 0, MAX_VOLUME);
    }

    pub fn dec_music_vol(&mut self) {
        self.music_volume -= 5;
        limit_range(&mut self.music_volume, 0, MAX_VOLUME);
    }

    pub fn get_music_vol(&self) -> i16 {
        self.music_volume
    }

    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }

    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    pub fn inc_resolution(&mut self) {
        self.resolution += 1;
        limit_range(&mut self.resolution, 0, MAX_RESOLUTION);
    }

    pub fn dec_resolution(&mut self) {
        self.resolution -= 1;
        limit_range(&mut self.resolution, 0, MAX_RESOLUTION);
    }

    pub fn get_resolution(&self) -> i16 {
        self.resolution
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

fn limit_range(value: &mut i16, low: i16, high: i16) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}
