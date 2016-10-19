extern crate csv;
extern crate nalgebra as na;
extern crate ncollide;

use entity::{Entity, EntityType};
use na::{Point2, Vector2};
use std::rc::Rc;
use std::cell::RefCell;
use entity::CollideWorld;
use graphics_component::GraphicsComponent;

pub type Row = [f32; 11];

#[derive(Clone,Debug)]
pub struct LevelReader {
    pub data: Vec<Row>
}

impl LevelReader {
    pub fn new(path: &str) -> Self {
        let mut rdr = csv::Reader::from_file(path).unwrap();
        let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();

        LevelReader {
            data: rows
        }
    }

    pub fn load_level(&self, world: &Rc<RefCell<CollideWorld>>) -> Vec<Entity> {
        self.data.iter().enumerate().map(|(i, row)| {
            let vector = match (row[8], row[9]) {
                (0.0, 0.0) => None,
                (a, b) => Some(Vector2::new(a, b))
            };
            let entity_type = if row[10] == 0.0 {
                EntityType::Character
            } else {
                EntityType::Wall
            };
            let graphics_component = graphics_component_for_type(&entity_type);
            Entity::new(Point2::new(row[0], row[1]), [row[2], row[3], row[4], row[5]], row[6], row[7], vector, world.clone(), i, entity_type, graphics_component)
        }).collect()
    }

}

fn graphics_component_for_type(entity_type: &EntityType) -> GraphicsComponent {
    match *entity_type {
        EntityType::Character => GraphicsComponent{sprite_filename: Some(String::from("./assets/green-blob-hi.png"))},
        EntityType::Wall      => GraphicsComponent{sprite_filename: None}
    }
}
