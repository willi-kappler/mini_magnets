// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game::{Game};
use crate::draw_text::{draw_text, draw_text_centered, Font};

pub struct MenuItems {
    selected_item: u8,
}

pub fn new_menu_items() -> MenuItems {
    MenuItems {
        selected_item: 1,
    }
}

pub fn process_main_menu(event: Event, quit: &mut bool, menu_items: &mut MenuItems) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            if menu_items.selected_item > 1 {
                menu_items.selected_item -= 1;
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            if menu_items.selected_item < 6 {
                menu_items.selected_item += 1;
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
            match menu_items.selected_item {
                1 => {
                    // Start game
                },
                2 => {
                    // Audio options
                },
                3 => {
                    // GFX options
                },
                4 => {
                    // Controls
                },
                5 => {
                    // High Score
                },
                6 => {
                    // Exit
                    *quit = true;
                }
                _ => {
                    unreachable!();
                }
            }
        },
        _ => {}
    }
}

pub fn update_main_menu(menu_items: &mut MenuItems) {

}

pub fn draw_main_menu(canvas: &mut Canvas<Window>, fonts: &Vec<Font>, elapsed: i64, menu_items: &MenuItems) {
    let font = &fonts[0];

    let fps_string = format!("FPS: {:2.2}", 1000.0 / (elapsed as f64));
    draw_text(canvas, font, 0, 0, &fps_string);
    let elapsed_string = format!("ELAPSED: {}", elapsed);
    draw_text(canvas, font, 400, 0, &elapsed_string);

    let menu = vec!["MAIN MENU", "START", "AUDIO OPTIONS", "GFX OPTIONS", "CONTROLS", "HIGH SCORE", "EXIT"];
    draw_menu(canvas, font, 400, 125, 30, menu, menu_items.selected_item as usize);
}

fn draw_menu(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, step: u32, texts: Vec<&str>, selected_item: usize) {
    let mut py = y;
    for (i, line) in texts.iter().enumerate() {
        if selected_item == i {
            let line = format!("-> {} <-", line);
            draw_text_centered(canvas, font, x, py, &line);
        } else {
            draw_text_centered(canvas, font, x, py, line);
        }
        py += step;
    }
}
