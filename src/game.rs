// Rust modules
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread;
use std::rc::Rc;

// External modules
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;

// Local modules
use crate::menu::{MainMenu};
use crate::draw_text::{Font};

pub struct Game {
    pub quit: bool,
    game_screen: GameScreen,
    game_settings: GameSettings,
    main_menu: MainMenu,
    frame_duration: i64,
    fps: u32,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    fonts: Vec<Rc<Font>>,
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

        Game {
            quit: false,
            game_screen: GameScreen::MainMenu,
            game_settings: GameSettings::new(),
            main_menu: MainMenu::new(),
            frame_duration: 16,
            fps: 0,
            canvas: canvas,
            event_pump: event_pump,
            texture_creator: texture_creator,
            fonts: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.load_resources();

        while !self.quit {
            let instant = Instant::now();

            self.process();
            self.update();
            self.draw();

            let sleep_time = self.frame_duration - (instant.elapsed().as_millis() as i64);
            if sleep_time > 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64))
            }

            self.calculate_fps(instant.elapsed().as_millis());
        }
    }

    fn process(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    // User closed main window, quit game
                    self.quit = true;
                },
                _ => {
                    match self.game_screen {
                        GameScreen::MainMenu => {
                            self.main_menu.process(&event, &mut self.quit);
                        },
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        match self.game_screen {
            GameScreen::MainMenu => {
                self.main_menu.update(self.fps);
            },
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        match self.game_screen {
            GameScreen::MainMenu => {
                self.main_menu.draw(&mut self.canvas)
            },
        }

        self.canvas.present();
    }

    fn calculate_fps(&mut self, elapsed: u128) {
        let fps = 1000.0 / (elapsed as f64);
        self.fps = (((self.fps as f64) + fps) / 2.0) as u32;
    }

    fn load_resources(&mut self) {
        self.load_font("assets/font2.png", 24, 24);

        self.main_menu.set_font(Rc::clone(&self.fonts[0]));
    }

    fn load_font<T: AsRef<Path>>(&mut self, path: T, char_width: u32, char_height: u32) {
        let texture = self.texture_creator.load_texture(path).unwrap();
        let texture_properties = texture.query();

        let font = Rc::new(Font {
            width: char_width,
            height: char_height,
            rows: (texture_properties.height / char_height) as u8,
            cols: (texture_properties.width / char_width) as u8,
            texture: texture,
        });

        self.fonts.push(font);
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

#[derive(Debug)]
pub enum GameScreen {
    MainMenu,
    /*
    AudioMenu,
    GFXMenu,
    ControlsMenu,
    HighScoreMenu,
    */
}
