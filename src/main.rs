// Rust modules
use std::time::Duration;

// External modules
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;

// Local modules
mod game_state;
mod menu;

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
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game_state = GameState {
        quit: false,
        game_screen: GameScreen::Menu,
    };

    while !game_state.quit {
        process(&mut game_state, &mut event_pump);
        update(&mut game_state);
        draw(&mut game_state, &mut canvas);
    }

    /*
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    */
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
    match game_state.game_screen {
        GameScreen::Menu => {
            menu::draw(canvas);
        },
        _ => {
            
        }
    }

    canvas.present();
}
