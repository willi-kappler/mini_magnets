// Rust modules
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread;

// External modules
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::EventPump;
use sdl2::pixels::Color;

// Local modules
use crate::menu::{MainMenu};
use crate::draw_text::{Font};
use crate::game_screen::{GameScreen};

pub struct Game {
    pub quit: bool,
    game_screens: Vec<Box<dyn GameScreen>>,
    current_screen: usize,
    game_settings: GameSettings,
    pub elapsed: i64,
    pub frame_duration: i64,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    pub fonts: Vec<Font>,
}

impl Game {
    pub fn new() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("Mini-Magnets", 800, 600)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        // let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
    
        // Activate support fot PNG and JPG image file format
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump().unwrap();

        let mut game_screens = Vec::new();
        game_screens.push(Box::new(MainMenu::new()) as Box<dyn GameScreen> );

        Game {
            quit: false,
            game_screens: game_screens,
            current_screen: 0,
            game_settings: GameSettings::new(),
            elapsed: 0,
            frame_duration: 16,
            canvas: canvas,
            event_pump: event_pump,
            texture_creator: texture_creator,
            fonts: Vec::new(),
        }
    }

    fn load_resources(&mut self) {
        self.load_font("assets/font2.png", 24, 24);

    }

    fn load_font<T: AsRef<Path>>(&mut self, path: T, char_width: u32, char_height: u32) {
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

    pub fn run(&mut self) {
        self.load_resources();

        while !self.quit {
            let instant = Instant::now();

            self.process();
            self.update();
            self.draw();

            self.elapsed = instant.elapsed().as_millis() as i64;
            let sleep_time = self.frame_duration - self.elapsed;
            if sleep_time > 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64))
            }
        }
    }

    fn process(&mut self) {
        self.game_screens[self.current_screen].process(self)
    }

    fn update(&mut self) {
        self.game_screens[self.current_screen].update(self)
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.game_screens[self.current_screen].draw(self);

        self.canvas.present();
    }
}

#[derive(Debug)]
pub struct GameSettings {
    pub start_level: u8,
    pub sound_volume: u8,
    pub music_volume: u8,
    pub fullscreen: bool,
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
