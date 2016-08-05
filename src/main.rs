extern crate piston_window;

use piston_window::*;

#[derive(Debug)]
struct Point(f64, f64);

fn main() {
    let title = "Hello Piston!";
    let mut pos = Point(0.0, 0.0);
    let mut delta = Point(0.0, 0.0);

    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                rectangle([1.0, 0.0, 1.0, 0.7], [pos.0, pos.1, 100.0, 100.0], c.transform, g);
            });
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up)    => delta.1 = -1.0,
                Button::Keyboard(Key::Down)  => delta.1 =  1.0,
                Button::Keyboard(Key::Left)  => delta.0 = -1.0,
                Button::Keyboard(Key::Right) => delta.0 =  1.0,
                Button::Keyboard(Key::Space) => delta   = Point(0.0, 0.0),
                _ => ()
            }
        }

        if let Some(_) = e.update_args() {
            pos.0 += delta.0;
            pos.1 += delta.1;

        }
    }
}
