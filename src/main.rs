extern crate piston_window;

mod entity;

use piston_window::*;
use entity::Entity;

#[derive(Debug, Clone)]
pub struct Point(f64, f64);

const MU: f64 = 0.93;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let title = "Goober";
    let mut i = 0;
    let mut squares = [
        Entity::new(Point(0.0, 0.0), [0.3, 0.0, 0.7, 0.5], 25.0, 25.0),
        Entity::new(Point(100.0, 100.0), [0.7, 0.0, 0.3, 0.5], 25.0, 25.0),
    ];

    let mut window: PistonWindow = WindowSettings::new(title, [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        let font = "assets/FiraSans-Regular.ttf";
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                for sq in &squares {
                    rectangle(sq.color, sq.geometry(), c.transform, g);
                }

                let transform = c.transform.trans(40.0, 40.0);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    "Hello World!",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                );
            });
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up)    => squares[i].adjust_dy(-1.0),
                Button::Keyboard(Key::Down)  => squares[i].adjust_dy(1.0),
                Button::Keyboard(Key::Left)  => squares[i].adjust_dx(-1.0),
                Button::Keyboard(Key::Right) => squares[i].adjust_dx(1.0),
                Button::Keyboard(Key::Space) => if i == squares.len() - 1 { i = 0 } else { i += 1 },
                _ => ()
            }
        }

        if let Some(_) = e.update_args() {
            squares[i].nudge();
        }
    }
}
