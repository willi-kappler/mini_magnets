// Rust modules
use std::path::Path;

// External modules
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

// Local modules
use crate::menu::{MenuState};
use crate::draw_text::{Font};

pub struct GameState<'a> {
    pub quit: bool,
    pub game_screen: GameScreen,
    pub menu_state: MenuState,
    pub game_settings: GameSettings,
    pub sleep_time: i64,
    pub frame_duration: i64,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub fonts: Vec<Font<'a>>,
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

impl<'a> GameState<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> GameState<'a> {
        GameState {
            quit: false,
            game_screen: GameScreen::Menu,
            menu_state: MenuState::new(),
            game_settings: GameSettings::new(),
            sleep_time: 0,
            frame_duration: 16,
            texture_creator: texture_creator,
            fonts: Vec::new(),
        }
    }

    pub fn load_font<T: AsRef<Path>>(&mut self, path: T, char_width: u32, char_height: u32) {
        let texture = self.texture_creator.load_texture(path).unwrap();
        let texture_properties = texture.query();

        let font = Font {
            width: char_width,
            height: char_height,
            rows: (texture_properties.height / char_height) as u8,
            cols: (texture_properties.width / char_width) as u8,
            texture: texture,
        };

        self.fonts.push(font);
    }
}
