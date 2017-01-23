extern crate nalgebra as na;

use na::{Point2, Isometry2};
use ncollide::narrow_phase::{ContactHandler, ContactAlgorithm2};
use ncollide::world::CollisionObject2;
use entity::{Entity, EntityType};

pub struct VelocityBouncer;

impl ContactHandler<Point2<f32>, Isometry2<f32>, Entity> for VelocityBouncer {
    fn handle_contact_started(&mut self,
                              co1: &CollisionObject2<f32, Entity>,
                              co2: &CollisionObject2<f32, Entity>,
                              alg: &ContactAlgorithm2<f32>) {
        // NOTE: real-life applications would avoid this systematic allocation.
        let mut collector = Vec::new();
        alg.contacts(&mut collector);

        if let Some(ref vel) = *co1.data.velocity() {
            let normal = collector[0].normal;
            vel.set(vel.get() - 2.0 * na::dot(&vel.get(), &normal) * normal);
        }
        if let Some(ref vel) = *co2.data.velocity() {
            let normal = -collector[0].normal;
            vel.set(vel.get() - 2.0 * na::dot(&vel.get(), &normal) * normal);
        }

        if character_hits_goal(co1, co2) {
            let ref dead1 = *co1.data.dead();
            dead1.set(true);
            let ref dead2 = *co2.data.dead();
            dead2.set(true);
        }
    }

    fn handle_contact_stopped(&mut self,
                              _: &CollisionObject2<f32, Entity>,
                              _: &CollisionObject2<f32, Entity>) {
        // We don't care.
    }
}

fn character_hits_goal(co1: &CollisionObject2<f32, Entity>, co2: &CollisionObject2<f32, Entity>) -> bool {
    (co1.data.entity_type == EntityType::Character && co2.data.entity_type == EntityType::Goal) ||
    (co1.data.entity_type == EntityType::Goal && co2.data.entity_type == EntityType::Character)
}
