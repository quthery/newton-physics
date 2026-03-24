use crate::state::GravityModel;
use std::f32::consts::PI;
use kiss3d::prelude::Vec2;

const G: f32 = 4.0 * PI * PI;

pub struct Newtonian;

impl GravityModel for Newtonian {
    fn compute_forces(&self, objs: &[crate::state::Object]) -> Vec<kiss3d::prelude::Vec2> {
        let mut forces = vec![Vec2::ZERO; objs.len()];
        for i in 0..objs.len() {
            // println!("----- object: {} -----", i);
            // println!("pos: x: {}; y: {}", objs[i].node.position().x, objs[i].node.position().y);
            // println!("velocity: x: {}; y: {}", objs[i].velocity.x, objs[i].velocity.y);
            for j in 0..objs.len() {
                if i == j {
                    continue;
                }

                let distance = objs[j].node.position() - objs[i].node.position();

                let d = (distance.x.powi(2) + distance.y.powi(2)).sqrt();

                let dir = distance / d;
                let f = G * objs[i].mass * objs[j].mass / d.powi(2);

                forces[i] += dir * f;
            }
        }
        forces
    }
}