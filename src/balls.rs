use na::{Vector2, Translation2};
use ncollide::shape::{Ball, Plane};
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;
use std::ops::Deref;


pub fn create_the_world() -> World<f32> {
    let mut world = World::new();
    world.set_gravity(Vector2::new(0.0, 9.81));
    world
}


pub fn create_the_walls(world: &mut World<f32>) {
    /*
     * First plane
     */
    let mut rb = RigidBody::new_static(Plane::new(Vector2::new(-1.0, -1.0)), 0.3, 0.6);
    rb.append_translation(&Translation2::new(0.0, 10.0));
    world.add_rigid_body(rb);

    /*
     * Second plane
     */
    let mut rb = RigidBody::new_static(Plane::new(Vector2::new(1.0, -1.0)), 0.3, 0.6);
    rb.append_translation(&Translation2::new(0.0, 10.0));
    world.add_rigid_body(rb);
}


pub fn create_the_balls(world: &mut World<f32>) {
    let num = (100.0f32.sqrt()) as usize;
    let rad = 0.5;
    let shift = 2.5 * rad;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift * (num as f32) / 2.0;

    for i in 0usize..num {
        for j in 0usize..num {
            let x = i as f32 * 2.5 * rad - centerx;
            let y = j as f32 * 2.5 * rad - centery * 2.0 - 20.0;

            let mut rb = RigidBody::new_dynamic(Ball::new(rad), 1.0, 0.3, 0.6);

            rb.append_translation(&Translation2::new(x, y));

            world.add_rigid_body(rb);
        }
    }
}

pub fn get_balls(world: &World<f32>) -> Vec<(f32, f32, f32)> {
    let balls: Vec<(f32, f32, f32)> = vec![(1.0, 1.0, 1.9)];
    for ball in world.rigid_bodies() {
        // let item: RigidBody<f32> = ball.clone(
        let item: RigidBody<f32> = ball.clone();
        let center = item.center_of_mass();
        dump!(center.len());
        // dump!(ball.center_of_mass());
    }




    balls
}