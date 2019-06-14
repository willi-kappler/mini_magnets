// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Font<'a> {
    // Width of a single character
    pub width: u32,
    // Height of a single character
    pub height: u32,
    // Number of rows in font atlas
    pub rows: u8,
    // Number of cols in font atlas
    pub cols: u8,
    // The actual image containing the font pixel data
    pub texture: Texture<'a>,
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

pub fn draw_char(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, c: u8) {
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

pub fn draw_text(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, text: &str) {
    let mut px = x;

    for c in text.chars() {
        draw_char(canvas, font, px, y, c as u8);
        px += font.width;
    }
}

pub fn draw_text_centered(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, text: &str) {
    let len = text.len() as u32;
    let x2 = x - ((len * font.width) / 2);
    draw_text(canvas, font, x2, y, text);
}
