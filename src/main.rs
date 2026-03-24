use std::f32::consts::PI;

use kiss3d::prelude::*;

pub mod parse;

const AU: f32 = 100.0;
const MSUN: f32 = 100.0;
const SUN_MASS: f32 = MSUN;
const EARTH_RADIUS: f32 = 10.0;
const SUN_RADIUS: f32 = 109.0 * EARTH_RADIUS / 2.0;
const G: f32 = 4.0 * PI * PI;
const DT: f32 = 1.0;

struct Object {
    node: SceneNode2d,
    mass: f32,
    velocity: Vec2,
    aclr: Vec2,
    trail: Vec<Vec2>,
    color: Rgba<f32>,
}

fn draw_arrow(window: &mut Window, start: Vec2, end: Vec2, color: Rgba<f32>) {
    let dir = (end - start).normalize();
    let perp = Vec2::new(-dir.y, dir.x);

    let head_len = 10.0;
    let head_width = 5.0;

    let left = end - dir * head_len + perp * head_width;
    let right = end - dir * head_len - perp * head_width;

    window.draw_line_2d(end, left, color, 10.0);
    window.draw_line_2d(end, right, color, 10.0);
}

#[kiss3d::main]
async fn main() {
    let mut window = Window::new_with_size("newton gravity", 1280, 720).await;

    let mut camera = PanZoomCamera2d::new(Vec2::ZERO, 1.0);
    camera.rebind_drag_button(Some(MouseButton::Button3));
    let mut scene = SceneNode2d::empty();

    let objects = parse::parse_data();

    let mut parsed_objects: Vec<Object> = objects
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

    let mut all_circ = parsed_objects;
    let mut forces = vec![Vec2::ZERO; all_circ.len()];
    let mut frame_count = 0;
    const TRAIL_RECORD_INTERVAL: usize = 5;
    const MAX_TRAIL_LENGTH: usize = 500;
    let mut enable_trails = false;
    let mut enable_potencial = true;
    camera.set_zoom(0.25);

    camera.set_at(all_circ[0].node.position());
    println!("DRAG CAMERA ON THE KOLESIKO MISHI");
    while window.render_2d(&mut scene, &mut camera).await {
        {
            for f in forces.iter_mut() {
                *f = Vec2::ZERO;
            }
        }
        for i in 0..all_circ.len() {
            // println!("----- object: {} -----", i);
            // println!("pos: x: {}; y: {}", all_circ[i].node.position().x, all_circ[i].node.position().y);
            // println!("velocity: x: {}; y: {}", all_circ[i].velocity.x, all_circ[i].velocity.y);
            for j in 0..all_circ.len() {
                if i == j {
                    continue;
                }

                let distance = all_circ[j].node.position() - all_circ[i].node.position();

                let d = (distance.x.powi(2) + distance.y.powi(2)).sqrt();

                let dir = distance / d;
                let f = G * all_circ[i].mass * all_circ[j].mass / d.powi(2);

                forces[i] += dir * f;
            }
        }

        for i in 0..forces.len() {
            let m_i = all_circ[i].mass;

            let aclr = forces[i] / m_i; // acceleration
            all_circ[i].aclr = aclr;
            all_circ[i].velocity += aclr * DT;
        }

        for i in 0..forces.len() {
            let local_velocity = all_circ[i].velocity;
            all_circ[i]
                .node
                .translate(Vec2::new(local_velocity.x * DT, local_velocity.y * DT));
        }

        // potencial

        if enable_potencial {
            for i in 0..forces.len() {
                let pos = all_circ[i].node.position();
                window.draw_line_2d(
                    all_circ[i].node.position(),
                    all_circ[i].node.position() + all_circ[i].aclr * 1000.0,
                    Rgba::new(1.0, 0.2, 0.2, 1.0), //red
                    3.0,
                );
                // scene.add_rectangle(30.0, 30.0).set_position(all_circ[i].node.position() + all_circ[i].aclr*1000.0);
                window.draw_line_2d(
                    all_circ[i].node.position(),
                    all_circ[i].node.position() + all_circ[i].velocity * 50.0,
                    Rgba::new(0.2, 1.0, 0.2, 1.0), //green
                    3.0,
                );
                // scene.add_rectangle(30.0, 30.0).set_position(all_circ[i].node.position() + all_circ[i].velocity * 50.0);
                draw_arrow(&mut window, pos, pos + all_circ[i].velocity * 50.0, GREEN);
            }
        }

        // trails
        if enable_trails {
            if frame_count % TRAIL_RECORD_INTERVAL == 0 {
                for obj in all_circ.iter_mut() {
                    obj.trail.push(obj.node.position());
                    if obj.trail.len() > MAX_TRAIL_LENGTH {
                        obj.trail.remove(0);
                    }
                }
            }
            frame_count += 1;

            for obj in all_circ.iter() {
                if obj.trail.len() > 1 {
                    for i in 0..obj.trail.len() - 1 {
                        window.draw_line_2d(obj.trail[i], obj.trail[i + 1], obj.color, 2.0);
                    }
                }
            }
        }

        // ui
        window.draw_ui(|ctx| {
            egui::Window::new("DRAG ON THE COLESIKO MISHI")
                .default_width(300.0)
                .show(ctx, |ui| {
                    let names = [
                        "Sun", "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus",
                        "Neptune",
                    ];
                    for (i, obj) in all_circ.iter().enumerate() {
                        let pos = obj.node.position();
                        let vel = obj.velocity;
                        let name = names.get(i).copied().unwrap_or("Unknown");
                        ui.label(format!(
                            "{}: pos=({:.1},{:.1}) vel=({:.2},{:.2})",
                            name, pos.x, pos.y, vel.x, vel.y
                        ));
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Enable trails").clicked() {
                            enable_trails = !enable_trails
                        }
                        if ui
                            .button("Enable velocity and acceleration vector")
                            .clicked()
                        {
                            enable_potencial = !enable_potencial
                        }
                    });
                });
        });
    }
}
