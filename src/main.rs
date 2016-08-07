extern crate piston_window;

mod entity;

use piston_window::*;
use entity::Entity;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Point(f64, f64);

const MU: f64 = 1.0;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const WAIT_TIME: u32 = 300_000_000; //nanoseconds

fn main() {
    let title = "Goober";
    let mut inputs_submitted = false;
    let mut inputs = Vec::new();
    let mut timestamp = SystemTime::now();
    let mut i: usize = 0;
    let mut squares = [
        Entity::new(Point(0.0, 0.0), [0.3, 0.0, 0.7, 0.5], 25.0, 25.0),
        Entity::new(Point(100.0, 100.0), [0.7, 0.0, 0.3, 0.5], 25.0, 25.0),
    ];

    let mut window: PistonWindow = WindowSettings::new(title, [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let sprite_path = "./assets/green-blob-hi.png";
    let sprite = Texture::from_path(
            &mut window.factory,
            &sprite_path,
            Flip::None,
            &TextureSettings::new())
            .unwrap();

    let font = "assets/FiraSans-Regular.ttf";
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.8, 0.0, 0.8, 1.0], g);
                for sq in &squares {
                    image(&sprite, c.transform.trans(sq.geometry()[0], sq.geometry()[1]), g);
                }

                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    &format_inputs(&inputs),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(40.0, 40.0), g
                );
            });
        }

        if let Some(button) = e.press_args() {
            if !inputs_submitted {
                match button {
                    Button::Keyboard(Key::Up)     => inputs.push(button),
                    Button::Keyboard(Key::Down)   => inputs.push(button),
                    Button::Keyboard(Key::Left)   => inputs.push(button),
                    Button::Keyboard(Key::Right)  => inputs.push(button),
                    Button::Keyboard(Key::Space)  => inputs.push(button),
                    Button::Keyboard(Key::Return) => {inputs_submitted = true; inputs.reverse()},
                    _ => ()
                }
            }
        }

        if let Some(_) = e.update_args() {
            if inputs_submitted && wait_time_elapsed(timestamp) {
                timestamp = SystemTime::now();
                match inputs.pop() {
                    Some(button) => handle_input(button, &mut squares, &mut i),
                    None => ()
                }
            }
            for square in squares.iter_mut() {
                square.nudge();
            }
        }
    }

    fn handle_input(button: Button, squares: &mut [Entity], i: &mut usize) {
        match button {
            Button::Keyboard(Key::Up)     => squares[*i].adjust_dy(-1.0),
            Button::Keyboard(Key::Down)   => squares[*i].adjust_dy(1.0),
            Button::Keyboard(Key::Left)   => squares[*i].adjust_dx(-1.0),
            Button::Keyboard(Key::Right)  => squares[*i].adjust_dx(1.0),
            Button::Keyboard(Key::Space)  => if *i == squares.len() - 1 { *i = 0 } else { *i += 1 },
            _ => ()
        }
    }

    fn wait_time_elapsed(timestamp: SystemTime) -> bool {
        match timestamp.elapsed() {
            Ok(elapsed) => elapsed.subsec_nanos() >= WAIT_TIME,
            Err(e) => {println!("{:?}", e); false}
        }
    }

    fn format_inputs(inputs: &Vec<Button>) -> String {
        let mut string = String::from("");
        for input in inputs {
            match *input {
                Button::Keyboard(Key::Up)     => string.push_str("↑"),
                Button::Keyboard(Key::Down)   => string.push_str("↓"),
                Button::Keyboard(Key::Left)   => string.push_str("←"),
                Button::Keyboard(Key::Right)  => string.push_str("→"),
                Button::Keyboard(Key::Space)  => string.push_str("↔"),
                _ => ()
            }
        }
        string
    }
}
