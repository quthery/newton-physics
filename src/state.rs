use kiss3d::prelude::*;

pub struct Object {
    pub node: SceneNode2d,
    pub mass: f32,
    pub velocity: Vec2,
    pub aclr: Vec2,
    pub trail: Vec<Vec2>,
    pub color: Rgba<f32>,
}

pub struct GameState {
    pub frame_count: i32,
    pub window: Window,
    pub objs: Vec<Object>,
    pub enable_potencial: bool,
    pub enable_trails: bool,
}
impl GameState {
    pub fn new(window: Window, objs: Vec<Object>) -> Self {
        Self {
            window,
            objs,
            enable_potencial: true,
            enable_trails: false,
            frame_count: 0,
        }
    }
}

pub trait GravityModel {
    fn compute_forces(&self, objs: &[Object]) -> Vec<Vec2>;
}

pub trait Integrator {
    fn step(&self, objs: &mut Vec<Object>, forces: &[Vec2], dt: f32);
}

pub trait CollisionModel {
    fn resolve(&self, objs: &mut Vec<Object>);
}
