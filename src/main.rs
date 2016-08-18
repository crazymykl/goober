extern crate piston_window;
extern crate ncollide;
extern crate nalgebra as na;

mod entity;
mod velocity_bouncer;
mod level_reader;

use std::rc::Rc;
use std::cell::RefCell;
use piston_window::*;
use ncollide::world::CollisionWorld;
use velocity_bouncer::VelocityBouncer;
use level_reader::LevelReader;

const MU: f32 = 1.0;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let title = "Goober";
    let mut i = 0;
    let world = Rc::new(RefCell::new(CollisionWorld::new(0.02, true)));
    world.borrow_mut().register_contact_handler("VelocityBouncer", VelocityBouncer);
    let lr = LevelReader::new("levels/level-1.csv");
    let mut squares = lr.load_level(&world);
    let mut window: PistonWindow = WindowSettings::new(title, [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    while let Some(e) = window.next() {

        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                for sq in &squares {
                    rectangle(sq.color(), sq.geometry(), c.transform, g);
                }
            });
        }

        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::Up)    => squares[i].adjust_dy(-1.0),
                Button::Keyboard(Key::Down)  => squares[i].adjust_dy(1.0),
                Button::Keyboard(Key::Left)  => squares[i].adjust_dx(-1.0),
                Button::Keyboard(Key::Right) => squares[i].adjust_dx(1.0),
                Button::Keyboard(Key::Space) => loop  {
                    if i >= squares.len() - 1 { i = 0; } else { i += 1; }
                    if squares[i].velocity().is_some() { break; }
                },
                _ => ()
            }
        }

        if let Some(_) = e.update_args() {
            world.borrow_mut().update();
            for sq in &mut squares {
                sq.nudge();
            }
        }
    }
}
