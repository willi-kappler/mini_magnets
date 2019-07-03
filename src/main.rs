// Rust modules

// External modules

// Local modules
mod game;
mod settings;
mod text_fx;
mod menu;
mod main_menu;
mod audio_menu;
mod gfx_menu;
mod credit_menu;
mod high_score;

use game::{Game};

// Needed libraries:
// apt install libsdl2-2.0-0 libsdl2-dev libsdl2-gfx-1.0-0 libsdl2-gfx-dev libsdl2-image-2.0-0 libsdl2-image-dev libsdl2-mixer-2.0-0 libsdl2-mixer-dev libsdl2-ttf-2.0-0 libsdl2-ttf-dev

// Game loop:
// https://gameprogrammingpatterns.com/game-loop.html
// https://gamedev.stackexchange.com/questions/69753/game-loop-best-way-to-limit-the-fps
// http://www.java-gaming.org/topics/game-loops/24220/view.html
// https://bell0bytes.eu/the-game-loop/
// https://www.reddit.com/r/pcmasterrace/comments/29qcqr/an_explanation_of_game_loops_fps_and_delta_time/
// http://www.informit.com/articles/article.aspx?p=2167437&seqNum=3

// Key Codes:
// https://rust-sdl2.github.io/rust-sdl2/sdl2/keyboard/enum.Keycode.html

// Maybe use deleagte:
// https://github.com/chancancode/rust-delegate

// For audio playback use rodio:
// https://github.com/tomaka/rodio
// https://docs.rs/rodio/0.9.0/rodio/



pub fn main() {
     match Game::new() {
          Err(e) => {
               println!("SDL init error occured: {}", e);
          },
          Ok(mut game) => {
               match game.run() {
                    Err(e) => {
                         println!("An error occured: {}", e);
                    },
                    Ok(_) => {
                    }
               }
          }
     }
}
