use kiss3d::prelude::Vec2;

use crate::state::Integrator;

pub struct Euler;

impl Integrator for Euler {
    fn step(&self, objs: &mut Vec<crate::state::Object>, forces: &[kiss3d::prelude::Vec2], dt: f32) {
        for i in 0..forces.len() {
            let m_i = objs[i].mass;

            let aclr = forces[i] / m_i; // acceleration
            objs[i].aclr = aclr;
            objs[i].velocity += aclr * dt;
        }

        for i in 0..forces.len() {
            let local_velocity = objs[i].velocity;
            objs[i]
                .node
                .translate(Vec2::new(local_velocity.x * dt, local_velocity.y * dt));
        }    
    }
}