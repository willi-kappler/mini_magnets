// Rust modules
use core::f64::consts::PI;

// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Texture;

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

fn draw_char(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, c: u8) {
    if c < 32 || c > 95 {
        // Outside of character range
        return
    }

    let index = c - 32;

    let row = index / font.cols;
    let col = index - (row * font.cols);

    let w = font.width;
    let h = font.height;

    let source = Rect::new(((col as u32) * w) as i32, ((row as u32) * h) as i32, w, h);
    let destination = Rect::new(x as i32, y as i32, w, h);
    canvas.copy(&font.texture, Some(source), Some(destination)).unwrap();
}

fn center_text(text: &str, x: u32, font: &Font) -> u32 {
    let len = text.len() as u32;
    x - ((len * font.width) / 2)
}

pub struct StaticText {
    x: u32,
    y: u32,
    text: String,
    // TODO: Add  Font
}

impl StaticText {
    pub fn new(x: u32, y: u32, text: &str) -> StaticText {
        StaticText {
            x,
            y,
            text: text.to_string(),
        }
    }

    pub fn center(&mut self, font: &Font) {
        self.x = center_text(&self.text, self.x, font);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, font: &Font) {
        let mut x2 = text.x;

        for c in text.text.chars() {
            draw_char(canvas, font, x2, text.y, c as u8);
            x2 += font.width;
        }
    }

    pub set_text(&mut self, new_text: &str) {
        self.text = new_text.to_string();
    }

    // TODO: pub fn chars() -> impl Iterator {}
}

pub struct WaveText {
    text: StaticText,
    amplitude: f64,
    phase: f64,
    speed: f64,
    active: bool,
}

impl WaveText {
    pub fn new(x: u32, y: u32, amplitude: f64, speed: f64, text: &str) -> WaveText {
        WaveText {
            text: StaticText::new(x, y, text),
            amplitude,
            phase: 0.0,
            speed,
            active: true,
        }
    }

    pub fn center(&mut self, font: &Font) {
        self.text.center(font);
    }

    pub draw(&self, canvas: &mut Canvas<Window>, font: &Font) {
        if self.active {
            let mut x2 = x;
            let mut y2 = y;
            let mut phase: self.phase;

            for c in self.text.text.chars() {
                y2 = y + ((self.amplitude * phase.sin()) as u32);
                draw_char(canvas, font, x2, y2, c as u8);
                x2 += font.width;
                phase = phase + self.speed;
            }
        } else {
            self.text.draw(canvas, font);
        }
    }
}

pub struct SelectableText {
    text: StaticText,
    active: bool,
    // TODO
}
