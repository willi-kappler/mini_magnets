// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
use crate::game::{Game};
use crate::draw_text::{StaticText, WaveText};

#[derive(Debug)]
pub struct MenuItem {
    num_of_items: u8,
    selected_item: u8,
}

impl MenuItem {
    pub fn new(num_of_items: u8) -> MenuItems {
        num_of_items,
        selected_item: 1,
    }

    pub up(&mut self) {
        self.selected_item -= 1;
        if self.selected_item == 0 {
            self.selected_item = self.num_of_items;
        }
    }

    pub down(&mut self) {
        self.selected_item += 1;
        if self.selected_item > self.num_of_items {
            self.selected_item = 1;
        }
    }
}

struct BaseMenu {
    menu_item: MenuItem,
}

impl BaseMenu {
    fn new(num_of_items: u8) -> BaseMenu {
        BaseMenu {
            menu_item: MenuItem::new(num_of_items),
        }
    }

    fn process(&mut self, event: &Event) -> {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                self.menu_item.up();
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.menu_item.down();
            },
            _ => {}
        }
    }

}

pub struct MainMenu {
    base_menu: BaseMenu,
    title: WaveText,
    fps: StaticText,
    menu_texts: Vec<StaticText>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            base_menu: BaseMenu::new(6),
            title: WaveText::new(300, 125, 10.0, 0.1, "MAIN MENU"),
            fps: StaticText::new(0, 575, "FPS"),
            menu_texts: vec![
                StaticText::new(300, 150, "START"),
                StaticText::new(300, 175, "AUDIO OPTIONS"),
                StaticText::new(300, 200, "GFX OPTIONS"),
                StaticText::new(300, 225, "CONTROLS"),
                StaticText::new(300, 250, "HIGH SCORE"),
                StaticText::new(300, 275, "EXIT"),
            ],
        }
    }

    pub fn process(&mut self, event: &Event) {

    }

    pub fn update(&mut self) {

    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, fonts: &Vec<Font>) {

    }
}

pub fn process_main_menu(event: Event, quit: &mut bool, menu_items: &mut MenuItems) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            menu_items.selected_item -= 1;
            if menu_items.selected_item == 0 {
                menu_items.selected_item = 6;
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            menu_items.selected_item += 1;
            if menu_items.selected_item == 7 {
                menu_items.selected_item = 1;
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
