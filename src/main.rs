extern crate piston_window;

mod entity;

use piston_window::*;
use entity::Entity;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Point(f64, f64);

const MU: f64 = 1.0;
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 800;
const WAIT_TIME: u32 = 300_000_000; //nanoseconds

fn main() {
    let title = "Goober";
    let mut inputs_submitted = false;
    let mut inputs = Vec::new();
    let mut timestamp = SystemTime::now();
    let mut i: usize = 0;
    let mut goobs = vec![
        Entity::new(Point(0.0, 0.0), [0.3, 0.0, 0.7, 0.5], 25.0, 25.0)
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
                for goob in &goobs {
                    image(&sprite, c.transform.trans(goob.geometry()[0], goob.geometry()[1]), g);
                }

                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16).draw(
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
                    Button::Keyboard(Key::A)      => {
                        goobs.push(
                            Entity::new(
                            Point(0.0, 0.0),
                            [0.3, 0.0, 0.7, 0.5],
                            25.0, 25.0
                            )
                        );
                        inputs.push(button);
                    },
                    Button::Keyboard(Key::Return) => {inputs_submitted = true; inputs.reverse()},
                    _ => ()
                }
            }
        }

        if let Some(_) = e.update_args() {
            if inputs_submitted && wait_time_elapsed(timestamp) {
                timestamp = SystemTime::now();
                match inputs.pop() {
                    Some(button) => handle_input(button, &mut goobs, &mut i),
                    None => ()
                }
            }
            for goob in goobs.iter_mut() {
                goob.nudge();
            }
        }
    }

    fn handle_input(button: Button, goobs: &mut [Entity], i: &mut usize) {
        match button {
            Button::Keyboard(Key::Up)     => goobs[*i].adjust_dy(-1.0),
            Button::Keyboard(Key::Down)   => goobs[*i].adjust_dy(1.0),
            Button::Keyboard(Key::Left)   => goobs[*i].adjust_dx(-1.0),
            Button::Keyboard(Key::Right)  => goobs[*i].adjust_dx(1.0),
            Button::Keyboard(Key::Space)  => if *i == goobs.len() - 1 { *i = 0 } else { *i += 1 },
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
        inputs.iter().map(|button|
            match *button {
                Button::Keyboard(Key::Up)    => "↑",
                Button::Keyboard(Key::Down)  => "↓",
                Button::Keyboard(Key::Left)  => "←",
                Button::Keyboard(Key::Right) => "→",
                Button::Keyboard(Key::Space) => "↔",
                Button::Keyboard(Key::A)     => "+",
                _ => ""
            }
        ).collect::<Vec<_>>().concat()
    }
}
