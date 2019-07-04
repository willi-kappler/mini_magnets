// Rust modules
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread;
use std::rc::Rc;
use std::error;
use std::fmt;
use std::io::Error as StdIOError;

// External modules
use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::{Window, WindowContext, WindowBuildError};
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::IntegerOrSdlError;

// Local modules
use crate::settings::{GameSettings};
use crate::text_fx::{Font};
use crate::main_menu::{MainMenu};
use crate::audio_menu::{AudioMenu};
use crate::gfx_menu::{GFXMenu};
use crate::high_score::{HighScoreMenu};
use crate::credit_menu::{CreditMenu};

pub struct Game {
    pub quit: bool,
    screen: GameScreen,
    settings: GameSettings,
    main_menu: MainMenu,
    audio_menu: AudioMenu,
    gfx_menu: GFXMenu,
    high_score_menu: HighScoreMenu,
    credit_menu: CreditMenu,
    frame_duration: i64,
    fps: u32,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    fonts: Vec<Rc<Font>>,
}

impl Game {
    pub fn new() -> Result<Game, GameError> {
        let sdl_context = sdl2::init()
            .map_err(|e| GameError::SDLInit(e))?;
        let video_subsystem = sdl_context.video()
            .map_err(|e| GameError::SDLVideo(e))?;

        let window = video_subsystem.window("Mini-Magnets", 800, 600)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().accelerated().build()?;
        // let mut canvas = window.into_canvas().build().map_err(|e| GameError::SDLCanvas(e))?;
        let texture_creator = canvas.texture_creator();

        // Activate support fot PNG and JPG image file format
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
            .map_err(|e| GameError::SDLImage(e))?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = sdl_context.event_pump()
            .map_err(|e| GameError::SDLEventPump(e))?;

        Ok(Game {
            quit: false,
            screen: GameScreen::new(),
            settings: GameSettings::new(),
            main_menu: MainMenu::new(),
            audio_menu: AudioMenu::new(),
            gfx_menu: GFXMenu::new(),
            high_score_menu: HighScoreMenu::new(),
            credit_menu: CreditMenu::new(),
            frame_duration: 16,
            fps: 0,
            canvas: canvas,
            event_pump: event_pump,
            texture_creator: texture_creator,
            fonts: Vec::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), GameError> {
        self.load_resources()?;

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

        match self.high_score_menu.save() {
            Err(e) => {
                println!("Could not save high score table: {}", e);
            },
            _ => {
            }
        }

        match self.settings.save() {
            Err(e) => {
                println!("Could not save settings: {}", e);
            },
            _ => {                
            }
        }

        Ok(())
    }

    fn process(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    // User closed main window, quit game
                    self.quit = true;
                },
                _ => {
                    match self.screen.current_screen {
                        GameScreenKind::MainMenu => {
                            self.main_menu.process(&event, &mut self.quit, &mut self.screen);
                        },
                        GameScreenKind::AudioMenu => {
                            self.audio_menu.process(&event, &mut self.screen, &mut self.settings);
                        },
                        GameScreenKind::GFXMenu => {
                            self.gfx_menu.process(&event, &mut self.screen, &mut self.settings);
                        },
                        GameScreenKind::HighScoreMenu => {
                            self.high_score_menu.process(&event, &mut self.screen);
                        },
                        GameScreenKind::CreditMenu => {
                            self.credit_menu.process(&event, &mut self.screen);
                        },
                        _ => {
                            println!("Not implemented yet!");
                            self.screen.current_screen = GameScreenKind::MainMenu;
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        match self.screen.current_screen {
            GameScreenKind::MainMenu => {
                self.main_menu.update(self.fps);
            },
            GameScreenKind::AudioMenu => {
                self.audio_menu.update();
            },
            GameScreenKind::GFXMenu => {
                self.gfx_menu.update();
            },
            GameScreenKind::HighScoreMenu => {
                self.high_score_menu.update();
            },
            GameScreenKind::CreditMenu => {
                self.credit_menu.update();
            },
            _ => {
                println!("Not implemented yet!");
                self.screen.current_screen = GameScreenKind::MainMenu;
            }
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        match self.screen.current_screen {
            GameScreenKind::MainMenu => {
                self.main_menu.draw(&mut self.canvas)
            },
            GameScreenKind::AudioMenu => {
                self.audio_menu.draw(&mut self.canvas)
            },
            GameScreenKind::GFXMenu => {
                self.gfx_menu.draw(&mut self.canvas)
            },
            GameScreenKind::HighScoreMenu => {
                self.high_score_menu.draw(&mut self.canvas)
            },
            GameScreenKind::CreditMenu => {
                self.credit_menu.draw(&mut self.canvas)
            },
            _ => {
                println!("Not implemented yet!");
                self.screen.current_screen = GameScreenKind::MainMenu;
            }
        }

        self.canvas.present();
    }

    fn calculate_fps(&mut self, elapsed: u128) {
        let fps = 1000.0 / (elapsed as f64);
        self.fps = (((self.fps as f64) + fps) / 2.0) as u32;
    }

    fn load_resources(&mut self) -> Result<(), GameError> {
        self.load_font("assets/font2.png", 24, 24)?;

        self.main_menu.set_font(&self.fonts[0]);
        self.credit_menu.set_font(&self.fonts[0]);
        self.high_score_menu.set_font(&self.fonts[0]);
        self.audio_menu.set_font(&self.fonts[0]);
        self.gfx_menu.set_font(&self.fonts[0]);

        match self.high_score_menu.load() {
            Err(e) => {
                println!("Could not load high score talbe ({}), using default", e);
            },
            _ => {
            }
        }

        match self.settings.load() {
            Err(e) => {
                println!("Could not load settings ({}), using default", e);
            },
            _ => {
            }
        }

        self.audio_menu.update_settings(&self.settings);
        self.gfx_menu.update_settings(&self.settings);

        Ok(())
    }

    fn load_font<T: AsRef<Path>>(&mut self, path: T, char_width: u32, char_height: u32) -> Result<(), GameError> {
        let texture = self.texture_creator.load_texture(path)
            .map_err(|e| GameError::SDLTextureLoad(e))?;
        let texture_properties = texture.query();

        let font = Rc::new(Font {
            width: char_width,
            height: char_height,
            rows: (texture_properties.height / char_height) as u8,
            cols: (texture_properties.width / char_width) as u8,
            texture: texture,
        });

        self.fonts.push(font);

        Ok(())
    }
}

#[derive(Debug)]
pub struct GameScreen {
    current_screen: GameScreenKind,
}

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen {
            current_screen: GameScreenKind::MainMenu,
        }
    }

    pub fn main_menu(&mut self) {
        self.current_screen = GameScreenKind::MainMenu;
    }

    pub fn audio_options(&mut self) {
        self.current_screen = GameScreenKind::AudioMenu;
    }

    pub fn gfx_options(&mut self) {
        self.current_screen = GameScreenKind::GFXMenu;
    }

    pub fn controls(&mut self) {
        self.current_screen = GameScreenKind::ControlsMenu;
    }

    pub fn high_score(&mut self) {
        self.current_screen = GameScreenKind::HighScoreMenu;
    }

    pub fn credit(&mut self) {
        self.current_screen = GameScreenKind::CreditMenu;
    }

    pub fn start_game(&mut self) {
        self.current_screen = GameScreenKind::PlayGame;
    }
}

#[derive(Debug)]
enum GameScreenKind {
    MainMenu,
    AudioMenu,
    GFXMenu,
    ControlsMenu,
    HighScoreMenu,
    CreditMenu,
    PlayGame,
}

#[derive(Debug)]
pub enum GameError {
    IOError(StdIOError),
    SDLTextureLoad(String),
    SDLInit(String),
    SDLVideo(String),
    SDLWindow(WindowBuildError),
    SDLCanvas(IntegerOrSdlError),
    SDLImage(String),
    SDLEventPump(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::IOError(ref e) => {
                write!(f, "IO error while accessing the high score file: {}", e)
            },
            GameError::SDLTextureLoad(ref e) => {
                write!(f, "Could not load Texture: {}", e)
            },
            GameError::SDLInit(ref e) => {
                write!(f, "Could not initialize SDL: {}", e)
            },
            GameError::SDLVideo(ref e) => {
                write!(f, "Could not initialize SDL video: {}", e)
            },
            GameError::SDLWindow(ref e) => {
                write!(f, "Could not initialize SDL window: {}", e)
            },
            GameError::SDLCanvas(ref e) => {
                write!(f, "Could not initialize SDL canvas: {}", e)
            },
            GameError::SDLImage(ref e) => {
                write!(f, "Could not initialize SDL image: {}", e)
            },
            GameError::SDLEventPump(ref e) => {
                write!(f, "Could not initialize SDL event pump: {}", e)
            },
        }
    }
}

impl error::Error for GameError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            GameError::IOError(ref e) => {
                Some(e)
            },
            GameError::SDLWindow(ref e) => {
                Some(e)
            },
            GameError::SDLCanvas(ref e) => {
                Some(e)
            },
            _ => {
                None
            },
        }
    }
}

impl From<WindowBuildError> for GameError {
    fn from(e: WindowBuildError) -> GameError {
        GameError::SDLWindow(e)
    }
}

impl From<IntegerOrSdlError> for GameError {
    fn from(e: IntegerOrSdlError) -> GameError {
        GameError::SDLCanvas(e)
    }
}
