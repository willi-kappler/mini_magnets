// External modules
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Font {
    width: u32,
    height: u32,
    rows: u32,
    cols: u32,
    texture: Texture<'static>,
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

pub fn draw_char(canvas: &mut Canvas<Window>, current_font: &Font, x: u32, y: u32, char: u8) {
    let mut source = Rect::new(0, 0, current_font.width, current_font.height);

    match char {
        32 => {
            // Nothing to do
            return
        },
        48..=57 => {
            // Number

        },
        65..=90 => {
            // Alphabetic character

        }
        _ => {
            // Not supported character

        },
    }

    let destination = Rect::new(x as i32, y as i32, current_font.width, current_font.height);
    canvas.copy(&current_font.texture, Some(source), Some(destination));
}

pub fn draw_text(canvas: &mut Canvas<Window>, font_list: &Vec<Font>, font_index: u8, x: u32, y: u32, text: &str) {
    let current_font = &font_list[font_index as usize];


}
