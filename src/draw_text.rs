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
    pub x: u32,
    pub y: u32,
    pub text: String,
}

pub new_static_text(x: u32, y: u32, text: &str) -> StaticText {
    StaticText {
        x,
        y,
        text: text.to_string(),
    }
}

pub fn draw_static_text(canvas: &mut Canvas<Window>, font: &Font, text: &StaticText) {
    let mut x2 = text.x;

    for c in text.text.chars() {
        draw_char(canvas, font, x2, text.y, c as u8);
        x2 += font.width;
    }
}

pub fn center_static_text(text: &mut StaticText, font: &Font) {
    text.x = center_text(&text.text, text.x, font);
}

pub struct WaveText {
    pub text: StaticText,
    pub amplitude: f64,
    pub phase: f64,
    pub speed: f64,
}

pub fn draw_wave_text(canvas: &mut Canvas<Window>, font: &Font, text: &mut WaveText) {
    let mut x2 = x;
    let mut y2 = y;
    let mut phase: wave.phase;

    for c in text.chars() {
        draw_char(canvas, font, x2, y2, c as u8);
        x2 += font.width;
        y2 = y + ((wave.amplitude * phase.sin()) as u32);
        phase = phase + wave.speed;
    }
}

