use MU;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use na::{Point2, Vector2, Isometry2, Translation};
use na;
use ncollide::world::{GeometricQueryType, CollisionGroups, CollisionWorld};
use ncollide::shape::{Cuboid, ShapeHandle2};

pub type CollideWorld = CollisionWorld<Point2<f32>, Isometry2<f32>, Entity>;

#[derive(Clone)]
pub struct Entity {
    color: [f32; 4],
    velocity: Option<Rc<Cell<Vector2<f32>>>>,
    width: f32,
    height: f32,
    world: Rc<RefCell<CollideWorld>>,
    index: usize,
}


impl Entity {
    pub fn new(pos: Point2<f32>, color: [f32; 4],
        w: f32, h: f32, velocity: Option<Vector2<f32>>,
        world: Rc<RefCell<CollideWorld>>,
        idx: usize) -> Self {

        let entity = Entity {
            color: color,
            velocity: velocity.map(|v| Rc::new(Cell::new(v))),
            width: w,
            height: h,
            world: world.clone(),
            index: idx,
        };

        let rect = ShapeHandle2::new(Cuboid::new(Vector2::new(w / 2.0, h / 2.0)));

        let pos = Isometry2::new(pos.to_vector(), na::zero());

        let mut groups = CollisionGroups::new();
        groups.set_membership(&[1]);

        let mut world = world.borrow_mut();

        world.add(idx, pos, rect, groups, GeometricQueryType::Contacts(0.0), entity.clone());
        world.update();

        entity
    }

    pub fn geometry(&self) -> [f64; 4] {
        let (x, y) = self.position();

        [x as f64, y as f64, self.width as f64, self.height as f64]
    }

    pub fn color(&self) -> [f32; 4] {
        self.color
    }

    pub fn velocity(&self) -> &Option<Rc<Cell<Vector2<f32>>>> {
        &self.velocity
    }

    pub fn nudge(&mut self) {
        if let Some(ref delta_v) = self.velocity {
            let dv = delta_v.get();
            let mut world = self.world.borrow_mut();
            let pos = world.collision_object(self.index).unwrap().position;
            let new_pos = pos.append_translation(&dv);

            world.deferred_set_position(self.index, new_pos);
            delta_v.set(dv * MU);
        }
    }

    pub fn adjust_dx(&mut self, dx: f32) {
        if let Some(ref delta_v) = self.velocity {
            delta_v.set(delta_v.get() + Vector2::new(dx, 0.0));
        }
    }

    pub fn adjust_dy(&mut self, dy: f32) {
        if let Some(ref delta_v) = self.velocity {
            delta_v.set(delta_v.get() + Vector2::new(0.0, dy));
        }
    }

    fn position(&self) -> (f32, f32) {
        let world = self.world.borrow();
        let pos = world.collision_object(self.index).unwrap().position.translation;

        (pos.x - self.width / 2.0, pos.y - self.height / 2.0)
    }

}
