// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
// use crate::game::{Game};
use crate::draw_text::{Font, StaticText, WaveText, SelectableText};

#[derive(Debug)]
pub struct MenuItem {
    num_of_items: u8,
    selected_item: u8,
}

impl MenuItem {
    pub fn new(num_of_items: u8) -> MenuItem {
        MenuItem {
            num_of_items,
            selected_item: 0,
        }
    }

    pub fn up(&mut self) {
        if self.selected_item == 0 {
            self.selected_item = self.num_of_items;
        } else {
            self.selected_item -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.selected_item == self.num_of_items - 1 {
            self.selected_item = 0;
        } else {
            self.selected_item += 1;
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

    fn process(&mut self, event: &Event) {
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
    menu_texts: Vec<SelectableText>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            base_menu: BaseMenu::new(6),
            title: WaveText::new(300, 100, 10.0, 0.1, 0.5, "MAIN MENU"),
            fps: StaticText::new(0, 575, "FPS"),
            menu_texts: MainMenu::create_texts(300, 150, 30,
                vec!["START", "AUDIO OPTIONS", "GFX OPTIONS", "CONTROLS", "HIGH SCORE", "EXIT"]),
        }
    }

    fn create_texts(x: u32, y: u32, step: u32, texts: Vec<&str>) -> Vec<SelectableText> {
        let mut y2 = y;
        let mut result: Vec<SelectableText> = Vec::new();

        for text in texts {
            result.push(SelectableText::new(x, y2, text));
            y2 += step;
        }

        result
    }

    pub fn process(&mut self, event: &Event, quit: &mut bool) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                match self.base_menu.menu_item.selected_item {
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
            _ => {
                self.base_menu.process(event);
            }
        }
    }

    pub fn update(&mut self, fps: u32) {
        self.title.update();

        let fps_string = format!("FPS: {}", fps);
        self.fps.set_text(&fps_string);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, fonts: &Vec<Font>) {
        let font = &fonts[0];

        self.title.draw(canvas, font);
        self.fps.draw(canvas, font);

        for item in self.menu_texts.iter() {
            item.draw(canvas, font);
        }
    }
}
