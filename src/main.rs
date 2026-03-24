use kiss3d::prelude::*;
pub mod intergrator;
pub mod parse;
pub mod physics;
pub mod render;
pub mod state;

use state::{GameState, Object};
use crate::state::GravityModel;
use crate::state::Integrator;

#[kiss3d::main]
async fn main() {
    let window = Window::new_with_size("newton gravity", 1280, 720).await;
    let mut camera = PanZoomCamera2d::new(Vec2::ZERO, 1.0);
    camera.rebind_drag_button(Some(MouseButton::Button3));
    let mut scene = SceneNode2d::empty();

    let objects = parse::parse_data();

    let parsed_objects: Vec<Object> = objects
        .into_iter()
        .map(|o| {
            let col = Rgba::new(
                o.color.r as f32 / 255.0,
                o.color.g as f32 / 255.0,
                o.color.b as f32 / 255.0,
                o.color.a as f32 / 255.0,
            );
            let mut n = scene.add_circle(o.radius);
            n.set_color(col);
            n.set_position(Vec2::new(o.position_x, o.position_y));
            Object {
                node: n,
                mass: o.mass,
                velocity: Vec2::new(o.velocity_x, o.velocity_y),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        })
        .collect();
    let mut game_state: GameState = GameState::new(window, parsed_objects);

    camera.set_zoom(0.25);

    camera.set_at(game_state.objs[0].node.position());
    println!("DRAG CAMERA ON THE KOLESIKO MISHI");
    while game_state.window.render_2d(&mut scene, &mut camera).await {
        let forces = physics::newtonian::Newtonian.compute_forces(&game_state.objs);
        intergrator::euler::Euler.step(&mut game_state.objs, &forces, 1.0);

        if game_state.enable_potencial {
            render::potencial(&mut game_state);
        }

        if game_state.enable_trails {
            render::trails(&mut game_state);
        }

        render::ui(&mut game_state);
    }
}
