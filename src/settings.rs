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
            resolution: ScreenResolution::R_800_600,
            filename: "assets/settings.txt".to_string(),
        }
    }

    pub fn load(&mut self) {

    }

    pub fn save(&self) {

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
    R_800_600,
    R_1024_768,
    R_1280_1024,
    R_1900_1200,
}
