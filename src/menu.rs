// Rust modules
use std::rc::Rc;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// Local modules
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
    x: u32,
    menu_item: MenuItem,
    title: WaveText,
    text: Vec<StaticText>,
    menu: Vec<SelectableText>,
}

impl BaseMenu {
    pub fn new(x: u32, y: u32, step: u32, title: &str, text: Vec<&str>, menu: Vec<&str>) -> BaseMenu {
        let mut y2 = y;

        let title = WaveText::new(x, y2, (step as f64) / 2.0, 0.1, 0.5, title);
        y2 += 2 * step;

        let text = BaseMenu::create_text(x, &mut y2, step, text);

        y2 += step;

        let menu = BaseMenu::create_menu(x, &mut y2, step, menu);
        let menu_item = MenuItem::new(menu.len());

        BaseMenu {
            x,
            menu_item,
            title,
            text,
            menu,
        }
    }

    pub fn create_text(x: u32, y2: &mut u32, step: u32, text: Vec<&str>) -> Vec<StaticText> {
        let mut result: Vec<StaticText> = Vec::new();

        if text.is_empty() {
            return result
        }

        for item in text {
            let mut static_text = StaticText::new(x, *y2, item);
            result.push(static_text);
            *y2 += step;
        }

        result
    }

    pub fn create_menu(x: u32, y2: &mut u32, step: u32, menu: Vec<&str>) -> Vec<SelectableText> {
        let mut result: Vec<SelectableText> = Vec::new();

        if menu.is_empty() {
            return result
        }

        for item in menu {
            let mut selectable_text = SelectableText::new(x, *y2, 30, item);
            result.push(selectable_text);
            *y2 += step;
        }

        result[0].set_active(true);

        result
    }

    pub fn process(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                let old = self.menu_item.selected;
                self.menu_item.up();
                let new = self.menu_item.selected;

                if old != new {
                    self.menu[old].set_active(false);
                    self.menu[new].set_active(true);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                let old = self.menu_item.selected;
                self.menu_item.down();
                let new = self.menu_item.selected;

                if old != new {
                    self.menu[old].set_active(false);
                    self.menu[new].set_active(true);
                }
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
        self.title.update();

        for item in self.menu.iter_mut() {
            item.update();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.title.draw(canvas);

        for item in self.text.iter() {
            item.draw(canvas);
        }

        for item in self.menu.iter() {
            item.draw(canvas);
        }
    }

    pub fn set_font(&mut self, font: &Rc<Font>) {
        self.title.set_font(font);
        self.title.center();

        for item in self.text.iter_mut() {
            item.set_font(font);
        }

        self.align_text();

        for item in self.menu.iter_mut() {
            item.set_font(font);
            item.center();
        }
    }

    pub fn get_selected(&self) -> usize {
        self.menu_item.selected
    }

    fn align_text(&mut self) {
        if self.text.is_empty() {
            return
        }

        let longest_text = self.text.iter().max_by_key(|item| item.get_width()).unwrap();
        let new_x = self.x - (longest_text.get_width() / 2);

        for item in self.text.iter_mut() {
            item.set_x(new_x);
        }
    }
}
