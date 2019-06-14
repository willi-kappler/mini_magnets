// Rust modules
use std::time::{Instant, Duration};
use std::thread;

// External modules
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::image::InitFlag;

// Local modules
mod game_state;
mod menu;
mod draw_text;
mod text_fx;

use game_state::{GameState, GameScreen};

// Needed libraries:
// apt install libsdl2-2.0-0 libsdl2-dev libsdl2-gfx-1.0-0 libsdl2-gfx-dev libsdl2-image-2.0-0 libsdl2-image-dev libsdl2-mixer-2.0-0 libsdl2-mixer-dev libsdl2-ttf-2.0-0 libsdl2-ttf-dev

// Game loop:
// https://gameprogrammingpatterns.com/game-loop.html
// https://gamedev.stackexchange.com/questions/69753/game-loop-best-way-to-limit-the-fps
// http://www.java-gaming.org/topics/game-loops/24220/view.html
// https://bell0bytes.eu/the-game-loop/
// https://www.reddit.com/r/pcmasterrace/comments/29qcqr/an_explanation_of_game_loops_fps_and_delta_time/
// http://www.informit.com/articles/article.aspx?p=2167437&seqNum=3

pub fn main() {
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

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState::new(&texture_creator);
    game_state.load_font("assets/font1.png", 32, 32);

    let mut instant = Instant::now();
    let mut sleep_time = 0;

    while !game_state.quit {
        process(&mut game_state, &mut event_pump);

        update(&mut game_state);

        draw(&mut game_state, &mut canvas);

        game_state.elapsed = instant.elapsed().as_millis() as i64;
        sleep_time = game_state.frame_duration - game_state.elapsed;
        if sleep_time > 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64))
        }
        instant = Instant::now();
    }
}

fn process(game_state: &mut GameState, event_pump: &mut EventPump) {
    match game_state.game_screen {
        GameScreen::Menu => {
            menu::process(game_state, event_pump);
        },
        _ => {

        }
    }
}

fn update(game_state: &mut GameState) {
    match game_state.game_screen {
        GameScreen::Menu => {
            menu::update();
        },
        _ => {
            
        }
    }
}

fn draw(game_state: &mut GameState, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    match game_state.game_screen {
        GameScreen::Menu => {
            menu::draw(game_state, canvas);
        },
        _ => {
            
        }
    }

    canvas.present();
}
