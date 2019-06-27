
// Rust modules
use std::rc::Rc;


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
    num_of_items: usize,
    selected_item: usize,
}

impl MenuItem {
    pub fn new(num_of_items: usize) -> MenuItem {
        MenuItem {
            num_of_items,
            selected_item: 0,
        }
    }

    pub fn up(&mut self) {
        if self.selected_item == 0 {
            self.selected_item = self.num_of_items - 1;
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
    fn new(num_of_items: usize) -> BaseMenu {
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
    base: BaseMenu,
    title: WaveText,
    fps: StaticText,
    menu_items: Vec<SelectableText>,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        let mut menu_items = MainMenu::create_texts(300, 150, 30, 25,
                vec!["START", "AUDIO OPTIONS", "GFX OPTIONS", "CONTROLS", "HIGH SCORE", "EXIT"]);

        menu_items[0].set_active(true);

        MainMenu {
            base: BaseMenu::new(6),
            title: WaveText::new(300, 100, 10.0, 0.1, 0.5, "MAIN MENU"),
            fps: StaticText::new(0, 575, "FPS"),
            menu_items,
        }
    }

    fn create_texts(x: u32, y: u32, step: u32, max_offset: u32, texts: Vec<&str>) -> Vec<SelectableText> {
        let mut y2 = y;
        let mut result: Vec<SelectableText> = Vec::new();

        for text in texts {
            result.push(SelectableText::new(x, y2, max_offset, text));
            y2 += step;
        }

        result
    }

    pub fn process(&mut self, event: &Event, quit: &mut bool) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                match self.base.menu_item.selected_item {
                    0 => {
                        // Start game
                    },
                    1 => {
                        // Audio options
                    },
                    2 => {
                        // GFX options
                    },
                    3 => {
                        // Controls
                    },
                    4 => {
                        // High Score
                    },
                    5 => {
                        // Exit
                        *quit = true;
                    }
                    _ => {
                        unreachable!();
                    }
                }
            },
            _ => {
                let prev_selected = self.base.menu_item.selected_item;
                self.base.process(event);
                let new_selected = self.base.menu_item.selected_item;

                if prev_selected != new_selected {
                    self.menu_items[prev_selected].set_active(false);
                    self.menu_items[new_selected].set_active(true);
                }
            }
        }
    }

    pub fn update(&mut self, fps: u32) {
        self.title.update();

        let fps_string = format!("FPS: {}", fps);
        self.fps.set_text(&fps_string);

        self.menu_items[self.base.menu_item.selected_item].update();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.title.draw(canvas);
        self.fps.draw(canvas);

        for item in self.menu_items.iter() {
            item.draw(canvas);
        }
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.title.set_font(Rc::clone(&font));
        self.fps.set_font(Rc::clone(&font));

        for item in self.menu_items.iter_mut() {
            item.set_font(Rc::clone(&font));
        }

        // println!("rc font count: {}", Rc::strong_count(&font));
    }
}
