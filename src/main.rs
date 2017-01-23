extern crate piston_window;
extern crate ncollide;
extern crate nalgebra as na;

mod entity;
mod velocity_bouncer;
mod level_reader;
mod action;
mod render_system;
mod graphics_component;
mod win_or_lose_system;

use std::rc::Rc;
use std::cell::RefCell;
use piston_window::*;
use ncollide::world::CollisionWorld;
use velocity_bouncer::VelocityBouncer;
use level_reader::LevelReader;
use action::Action;
use std::time::SystemTime;
use std::path::Path;
use entity::{Entity, EntityType, CollideWorld};
use na::{Point2, Vector2, Translation};
use render_system::RenderSystem;
use graphics_component::GraphicsComponent;
use win_or_lose_system::{WinOrLoseSystem,VictoryState};

const MU: f32 = 0.99;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const WAIT_TIME: u32 = 300_000_000; //nanoseconds

fn main() {
    let title = "Goober";
    let mut level = 1;
    let mut i = 0;
    let mut spawn_i = 0;
    let mut inputs_submitted = false;
    let mut inputs = Vec::new();
    let mut timestamp = SystemTime::now();
    let mut world = Rc::new(RefCell::new(CollisionWorld::new(0.02, true)));
    world.borrow_mut().register_contact_handler("VelocityBouncer", VelocityBouncer);
    let mut lr = LevelReader::new("levels/level-1.csv");
    let mut squares = lr.load_level(&world);
    let mut goobs = squares.clone();
    goobs.retain(|square| square.entity_type == EntityType::Character);
    let mut walls = squares.clone();
    walls.retain(|square| square.entity_type != EntityType::Character);
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

                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 16).draw(
                    &format_inputs(&inputs),
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(40.0, 40.0), g
                );
            });
            for wall in &walls {
                RenderSystem::render_entity(&wall, &mut window, &e);
            }
            for goob in &goobs {
                RenderSystem::render_entity(&goob, &mut window, &e);
            }
        }

        if let Some(button) = e.press_args() {
            if !inputs_submitted {
                match button {
                    Button::Keyboard(Key::Up)     => inputs.push(Action::Up),
                    Button::Keyboard(Key::Down)   => inputs.push(Action::Down),
                    Button::Keyboard(Key::Left)   => inputs.push(Action::Left),
                    Button::Keyboard(Key::Right)  => inputs.push(Action::Right),
                    Button::Keyboard(Key::Space)  => inputs.push(Action::Swap),
                    Button::Keyboard(Key::A)      => inputs.push(Action::Spawn),
                    Button::Keyboard(Key::Return) => {inputs_submitted = true; inputs.reverse()},
                    _ => ()
                }
            }
        }

        if let Some(_) = e.update_args() {
            if inputs_submitted && wait_time_elapsed(timestamp) {
                timestamp = SystemTime::now();
                if let Some(button) = inputs.pop() { handle_input(button, &mut goobs, &mut i, &world, &mut spawn_i, &squares); }
            } else if inputs.is_empty() {
                inputs_submitted = false;
            }
            world.borrow_mut().update();
            for goob in &mut goobs {
                goob.nudge();
            }
            goobs.retain(|entity|
                if entity.dead().get() {
                    let pos = world.borrow_mut().collision_object(entity.index).unwrap().position;
                    let new_pos = pos.append_translation(&Vector2{x: -1000.0, y: -1000.0});
                    world.borrow_mut().deferred_set_position(entity.index, new_pos); // move the goob offscreen until the level ends
                    i = 0;
                    false
                } else {
                    true
                }
            );
            walls.retain(|entity|
                if entity.dead().get() {
                    world.borrow_mut().deferred_remove(entity.index);
                    false
                } else {
                    true
                }
            );
            let win_or_loss = WinOrLoseSystem::win_or_loss(&goobs, &walls);
            match win_or_loss {
                Some(victory_state) => {
                    match victory_state {
                        VictoryState::Win  => {
                            level += 1;
                            let next_level = format!("levels/level-{}.csv", level);
                            if Path::new(&next_level).exists() {
                                load_level(next_level, &mut world, &mut lr, &mut squares, &mut goobs, &mut walls, &mut inputs_submitted, &mut inputs);
                            } else {
                                std::process::exit(0);
                            }
                        }
                        VictoryState::Loss => {
                            let current_level = format!("levels/level-{}.csv", level);
                            load_level(current_level, &mut world, &mut lr, &mut squares, &mut goobs, &mut walls, &mut inputs_submitted, &mut inputs);
                        }
                    }
                },
                None => {}
            }
        }
    }

    fn load_level(level_name: String, world: &mut Rc<RefCell<CollideWorld>>, lr: &mut LevelReader, squares: &mut Vec<Entity>, goobs: &mut Vec<Entity>, walls: &mut Vec<Entity>, inputs_submitted: &mut bool, inputs: &mut Vec<Action>) {
        *world = Rc::new(RefCell::new(CollisionWorld::new(0.02, true)));
        world.borrow_mut().register_contact_handler("VelocityBouncer", VelocityBouncer);
        *lr = LevelReader::new(&level_name);
        *squares = lr.load_level(&world);
        *goobs = squares.clone();
        goobs.retain(|square| square.entity_type == EntityType::Character);
        *walls = squares.clone();
        walls.retain(|square| square.entity_type != EntityType::Character);
        *inputs_submitted = false;
        inputs.clear();
    }

    fn handle_input(action: Action, goobs: &mut Vec<Entity>, i: &mut usize, world: &Rc<RefCell<CollideWorld>>, spawn_i: &mut usize, squares: &Vec<Entity>) {
        if !goobs.is_empty() {
            match action {
                Action::Up    |
                Action::Down  |
                Action::Left  |
                Action::Right  => goobs[*i].handle_input(action),
                Action::Swap   => if *i == goobs.len() - 1 { *i = 0 } else { *i += 1 },
                Action::Spawn  => {
                    *spawn_i += 1;
                    let new_idx = squares.len() + *spawn_i;
                    spawn_goob(goobs, i, world.clone(), new_idx);
                }
            }
        }
    }

    fn spawn_goob(goobs: &mut Vec<Entity>, i: &mut usize, world: Rc<RefCell<CollideWorld>>, new_idx: usize) {
        let new_position = Point2::new(goobs[*i].x_pos() as f32, goobs[*i].y_pos() as f32);
        goobs.push(
            Entity::new(
                new_position,
                [0.3, 0.0, 0.7, 0.5],
                25.0,
                25.0,
                Some(Vector2::new(0.0, 0.0)),
                world,
                new_idx,
                EntityType::Character,
                GraphicsComponent{sprite_filename: Some(String::from("./assets/green-blob-hi.png"))}
            )
        );
    }

    fn wait_time_elapsed(timestamp: SystemTime) -> bool {
        match timestamp.elapsed() {
            Ok(elapsed) => elapsed.subsec_nanos() >= WAIT_TIME,
            Err(e) => {println!("{:?}", e); false}
        }
    }

    fn format_inputs(inputs: &[Action]) -> String {
        inputs.iter().cloned().map(|action| action.to_string()).collect::<Vec<_>>().concat()
    }
}
