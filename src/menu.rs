// Rust modules
use std::rc::Rc;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
// use crate::game::{Game};
use crate::text_fx::{Font, StaticText, WaveText, SelectableText};

#[derive(Debug)]
pub struct MenuItem {
    num_of_items: usize,
    selected: usize,
}

impl MenuItem {
    pub fn new(num_of_items: usize) -> MenuItem {
        MenuItem {
            num_of_items,
            selected: 0,
        }
    }

    pub fn up(&mut self) {
        if self.selected == 0 {
            self.selected = self.num_of_items - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.selected == self.num_of_items - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }
}

pub struct BaseMenu {
    menu_item: MenuItem,
}

impl BaseMenu {
    pub fn new(num_of_items: usize) -> BaseMenu {
        BaseMenu {
            menu_item: MenuItem::new(num_of_items),
        }
    }

    pub fn create_texts(x: u32, y: u32, step: u32, max_offset: u32, texts: Vec<&str>) -> Vec<SelectableText> {
        let mut y2 = y;
        let mut result: Vec<SelectableText> = Vec::new();

        for text in texts {
            result.push(SelectableText::new(x, y2, max_offset, text));
            y2 += step;
        }

        result[0].set_active(true);

        result
    }

    pub fn process(&mut self, event: &Event) {
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

    pub fn get_selected(&self) -> usize {
        self.menu_item.selected
    }
}
