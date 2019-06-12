

// Local modules
use crate::menu::{MenuState};

pub struct GameState {
    pub quit: bool,
    pub game_screen: GameScreen,
    pub menu_state: MenuState,
    pub game_settings: GameSettings,
    pub sleep_time: i64,
    pub frame_duration: i64,
}

pub enum GameScreen {
    Menu,
    Game,
    NextLevel,
    Pause,
    GameOver,
    QuitConfirm,
}

pub struct GameSettings {
    start_level: u8,
    sound_volume: u8,
    music_volume: u8,
    fullscreen: bool,
}

impl GameSettings {
    pub fn new() -> GameSettings {
        GameSettings {
            start_level: 0,
            sound_volume: 255,
            music_volume: 255,
            fullscreen: false,
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            quit: false,
            game_screen: GameScreen::Menu,
            menu_state: MenuState::new(),
            game_settings: GameSettings::new(),
            sleep_time: 0,
            frame_duration: 16,
        }
    }
}
