// Rust modules
// use std::rc::Rc;

// External modules
// use sdl2::render::Canvas;
// use sdl2::video::Window;
// use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Font2 {
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

pub trait TextFX {
    fn set_text(&mut self, new_text: String) {
    }

    fn set_pos(x: u32, y: u32) {
    }

    fn draw(&mut self) {
    }

    fn reset(&mut self) {
    }

    fn next_step(&mut self) {
    }
}

#[derive(Debug)]
pub struct FXStatic {
    //font: Rc<Font2>,
    //canvas: Canvas,
    text: String,
    x: u32,
    y: u32,
}

impl FXStatic {
    pub fn new(text: String, x: u32, y: u32) -> FXStatic {
        FXStatic {
            text: text,
            x: x,
            y: y,
        }
    }
}

impl TextFX for FXStatic {

}

#[derive(Debug)]
pub struct FXSineWave {
    phase: f32,
    amplitude: f32,
    speed: f32,
}

impl FXSineWave {
    pub fn new(amplitude: u32, speed: f32) -> FXSineWave {
        FXSineWave {
            phase: 0.0,
            amplitude: amplitude as f32,
            speed: speed,
        }
    }
}

impl TextFX for FXSineWave {

}
