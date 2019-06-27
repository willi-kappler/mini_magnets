// Rust modules
use core::f64::consts::PI;
use std::rc::Rc;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Texture;

const PI_2: f64 = 2.0 * PI;

// ASCII table:
//
// https://de.wikipedia.org/wiki/American_Standard_Code_for_Information_Interchange
//
// 32   Space       33 !        34 "        35 #
// 36 $             37 %        38 &        39 '
// 40 (             41 )        42 *        43 +
// 44 ,             45 -        46 .        47 /
// 48 0 Zero        49 1        57 9        58 :
// 59 ;             60 <        61 =        62 >
// 63 ?             64 @        65 A        90 Z
// 91 [             92 \        93 ]        94 ^
// 95 _

pub struct Font {
    // Width of a single character
    pub width: u32,
    // Height of a single character
    pub height: u32,
    // Number of rows in font atlas
    pub rows: u8,
    // Number of cols in font atlas
    pub cols: u8,
    // The actual image containing the font pixel data
    pub texture: Texture,
}

impl Font {
    fn draw_char(&self, canvas: &mut Canvas<Window>, x: u32, y: u32, c: u8) {
        if c < 32 || c > 95 {
            // Outside of character range
            return
        }

        let index = c - 32;

        let row = index / self.cols;
        let col = index - (row * self.cols);

        let w = self.width;
        let h = self.height;

        let source = Rect::new(((col as u32) * w) as i32, ((row as u32) * h) as i32, w, h);
        let destination = Rect::new(x as i32, y as i32, w, h);
        canvas.copy(&self.texture, Some(source), Some(destination)).unwrap();
    }
}

pub struct StaticText {
    x: u32,
    y: u32,
    text: String,
    font: Option<Rc<Font>>,
}

impl StaticText {
    pub fn new(x: u32, y: u32, text: &str) -> StaticText {
        StaticText {
            x,
            y,
            text: text.to_string(),
            font: None,
        }
    }

    pub fn center(&mut self) {
        if let Some(font) = &self.font {
            let len = self.text.len() as u32;
            self.x = self.x - ((len * font.width) / 2);
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(font) = &self.font {
            let mut x2 = self.x;

            for c in self.text.chars() {
                font.draw_char(canvas, x2, self.y, c as u8);
                x2 += font.width;
            }
        }
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.text = new_text.to_string();
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.font = Some(font);
    }

    // TODO: pub fn chars() -> impl Iterator {}
}

pub struct WaveText {
    base: StaticText,
    amplitude: f64,
    phase: f64,
    speed: f64,
    shift: f64,
    active: bool,
}

impl WaveText {
    pub fn new(x: u32, y: u32, amplitude: f64, speed: f64, shift: f64, text: &str) -> WaveText {
        WaveText {
            base: StaticText::new(x, y, text),
            amplitude,
            phase: 0.0,
            speed,
            shift,
            active: true,
        }
    }

    pub fn center(&mut self) {
        self.base.center();
    }

    pub fn update(&mut self) {
        if self.active {
            self.phase += self.speed;
            if self.phase > PI_2 {
                self.phase -= PI_2;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(font) = &self.base.font {
            if self.active {
                let mut x2 = self.base.x;
                let mut y2;
                let mut phase = self.phase;

                for c in self.base.text.chars() {
                    y2 = self.base.y + ((self.amplitude * phase.sin()) as u32);
                    font.draw_char(canvas, x2, y2, c as u8);
                    x2 += font.width;
                    phase = phase + self.shift;
                }
            } else {
                self.base.draw(canvas);
            }
        }
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.base.set_text(new_text);
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.base.set_font(font);
    }
}

pub struct SelectableText {
    base: StaticText,
    active: bool,
}

impl SelectableText {
    pub fn new(x: u32, y: u32, text: &str) -> SelectableText {
        SelectableText {
            base: StaticText::new(x, y, text),
            active: false,
        }
    }

    pub fn center(&mut self) {
        self.base.center();
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.base.draw(canvas);

        if self.active {

        }
    }

    pub fn update(&mut self) {
        if self.active {

        }
    }

    pub fn set_text(&mut self, new_text: &str) {
        self.base.set_text(new_text);
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_font(&mut self, font: Rc<Font>) {
        self.base.set_font(font);
    }
}
