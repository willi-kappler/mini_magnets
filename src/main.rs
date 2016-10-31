// #[macro_use] extern crate quick_error;

extern crate piston_window;

use piston_window::*;

fn main() {
    println!("Hello World");

    let (screen_width, screen_height) = (640.0, 480.0);

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!",
        (screen_width as u32, screen_height as u32))
        .exit_on_esc(false)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut x = 0.0;
    let mut y = 0.0;

    let mut vx = 0.0;
    let mut vy = 0.0;

    let mut accelx = 0.0;
    let mut accely = 0.0;

    let accel = 0.01;

    let friction = 0.99;

    let (sizex, sizey) = (50.0, 50.0);

    let borderx = screen_width - sizex;
    let bordery = screen_height - sizey;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g); // black background
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [x, y, sizex, sizey], // rectangle
                      c.transform, g);
        });

        x = x + vx;
        y = y + vy;

        if x < 0.0 { vx = -vx; x = 0.0; }
        else if x > borderx { vx = -vx; x = borderx; }
        if y < 0.0 { vy = -vy; y = 0.0; }
        else if y > bordery { vy = -vy; y = bordery; }

        vx = vx + accelx;
        vy = vy + accely;

        vx = vx * friction;
        vy = vy * friction;

        match e.press_args() {
            Some(Button::Keyboard(Key::Up)) => accely = accely - accel,
            Some(Button::Keyboard(Key::Down)) => accely = accely + accel,
            Some(Button::Keyboard(Key::Left)) => accelx = accelx - accel,
            Some(Button::Keyboard(Key::Right)) => accelx = accelx + accel,
            _ => ()
        };

        match e.release_args() {
            Some(Button::Keyboard(Key::Up)) | Some(Button::Keyboard(Key::Down)) => accely = 0.0,
            Some(Button::Keyboard(Key::Left)) | Some(Button::Keyboard(Key::Right)) => accelx = 0.0,
            _ => ()
        };
    }
}
