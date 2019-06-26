// Rust modules
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread;

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
    pub game_screen: GameScreen,
    pub game_settings: GameSettings,
    pub main_menu: MainMenu,
    pub elapsed: i64,
    pub frame_duration: i64,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    texture_creator: TextureCreator<WindowContext>,
    pub fonts: Vec<Font>,
}

pub fn new_game() -> Game {
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
        game_settings: new_settings(),
        main_menu: MainMenu::new(),
        elapsed: 0,
        frame_duration: 16,
        canvas: canvas,
        event_pump: event_pump,
        texture_creator: texture_creator,
        fonts: Vec::new(),
    }
}

pub fn run_game(game: &mut Game) {
    load_resources(game);

    while !game.quit {
        let instant = Instant::now();

        process(game);
        update(game);
        draw(game);

        game.elapsed = instant.elapsed().as_millis() as i64;
        let sleep_time = game.frame_duration - game.elapsed;
        if sleep_time > 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64))
        }
    }
}

fn process(game: &mut Game) {
    for event in game.event_pump.poll_iter() {
        match event {
            Event::Quit {..} => {
                // User closed main window, quit game
                game.quit = true;
            },
            _ => {
                match game.game_screen {
                    GameScreen::MainMenu => {
                        game.main_menu.process(&event, &mut game.quit);
                    },
                }
            }
        }
    }
}

fn update(game: &mut Game) {
    match game.game_screen {
        GameScreen::MainMenu => {
            game.main_menu.update(game.elapsed);
        },
    }
}

fn draw(game: &mut Game) {
    game.canvas.set_draw_color(Color::RGB(0, 0, 0));
    game.canvas.clear();

    match game.game_screen {
        GameScreen::MainMenu => {
            game.main_menu.draw(&mut game.canvas, &game.fonts)
        },
    }

    game.canvas.present();
}

fn load_resources(game: &mut Game) {
    load_font(game, "assets/font2.png", 24, 24);
}

fn load_font<T: AsRef<Path>>(game: &mut Game, path: T, char_width: u32, char_height: u32) {
    let texture = game.texture_creator.load_texture(path).unwrap();
    let texture_properties = texture.query();

    let font = Font {
        width: char_width,
        height: char_height,
        rows: (texture_properties.height / char_height) as u8,
        cols: (texture_properties.width / char_width) as u8,
        texture: texture,
    };

    game.fonts.push(font);
}

fn new_settings() -> GameSettings {
    GameSettings {
        start_level: 0,
        sound_volume: 255,
        music_volume: 255,
        fullscreen: false,
    }
}


#[derive(Debug)]
pub struct GameSettings {
    pub start_level: u8,
    pub sound_volume: u8,
    pub music_volume: u8,
    pub fullscreen: bool,
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
