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

// let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
// https://docs.rs/sdl2/0.32.2/sdl2/surface/struct.Surface.html#method.load_bmp

// let texture = texture_creator.create_texture_from_surface(&temp_surface).map_err(|e| e.to_string())?;
// https://docs.rs/sdl2/0.32.2/sdl2/render/struct.TextureCreator.html#method.create_texture_from_surface

// canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false)?;
// canvas.copy(&texture, Some(source_rect_0), Some(dest_rect_0))?;
// https://docs.rs/sdl2/0.32.2/sdl2/render/struct.Canvas.html#method.copy



// ASCII table:
//
// https://de.wikipedia.org/wiki/American_Standard_Code_for_Information_Interchange
//
// 32   Space
// 48 0 Zero
// 49 1 One
// 57 9 Nine
// 65 A Capital A
// 90 Z Capital Z

pub fn draw_char(canvas: &mut Canvas<Window>, font: &Font, x: u32, y: u32, c: u8) {
    let index: u8;

    match c {
        32 => {
            // Nothing to do
            return
        },
        48..=57 => {
            // Number
            index = c - 48;
        },
        65..=90 => {
            // Alphabetic character
            index = c - 55;
        }
        _ => {
            // Not supported character
            return
        },
    }

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
