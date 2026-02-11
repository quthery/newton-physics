use std::f32::consts::PI;

use kiss3d::prelude::*;

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
    // главная линия
    let dir = (end - start).normalize();
    let perp = Vec2::new(-dir.y, dir.x);

    let head_len = 10.0;   // длина "крыльев"
    let head_width = 5.0;  // ширина

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

    let mut all_circ = vec![
        //  Sun
        {
            let mut n = scene.add_circle(SUN_RADIUS);
            let col = Rgba::new(1.0, 0.9, 0.2, 1.0); // Gold
            n.set_color(col);
            n.set_position(Vec2::ZERO);
            Object {
                node: n,
                mass: SUN_MASS,
                velocity: Vec2::ZERO,
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Mercury (Меркурий)
        {
            let orbital_distance = 0.387 * AU; 
            let r = SUN_RADIUS + orbital_distance; 
            let v = (G * SUN_MASS / r).sqrt(); 
            let mut n = scene.add_circle(0.383 * EARTH_RADIUS / 2.0); 
            let col = Rgba::new(0.6, 0.6, 0.6, 1.0); // Gray
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 0.055 * MSUN / 333000.0,
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Venus (Венера)
        {
            let orbital_distance = 0.723 * AU;
            let r = SUN_RADIUS + orbital_distance;
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(0.949 * EARTH_RADIUS / 2.0);
            let col = Rgba::new(0.9, 0.8, 0.5, 1.0); // Light Yellow
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 0.815 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Earth (Земля)
        {
            let orbital_distance = 1.0 * AU; 
            let r = SUN_RADIUS + orbital_distance;
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(EARTH_RADIUS / 2.0); 
            let col = Rgba::new(0.2, 0.4, 1.0, 1.0); // Blue
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 1.0 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Mars (Марс)
        {
            let orbital_distance = 1.524 * AU; 
            let r = SUN_RADIUS + orbital_distance; 
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(0.532 * EARTH_RADIUS / 2.0); 
            let col = Rgba::new(0.9, 0.3, 0.2, 1.0); // Red-Orange
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 0.107 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Jupiter (Юпитер)
        {
            let orbital_distance = 5.203 * AU; 
            let r = SUN_RADIUS + orbital_distance; 
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(11.21 * EARTH_RADIUS / 2.0); 
            let col = Rgba::new(0.9, 0.6, 0.3, 1.0); // Orange
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 317.8 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Saturn (Сатурн)
        {
            let orbital_distance = 9.537 * AU; 
            let r = SUN_RADIUS + orbital_distance; 
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(9.45 * EARTH_RADIUS / 2.0); 
            let col = Rgba::new(0.9, 0.8, 0.6, 1.0); // Pale Gold
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 95.2 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Uranus (Уран)
        {
            let orbital_distance = 19.191 * AU; 
            let r = SUN_RADIUS + orbital_distance; 
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(4.01 * EARTH_RADIUS / 2.0);
            let col = Rgba::new(0.4, 0.8, 0.9, 1.0); // Light Blue
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 14.5 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
        //  Neptune (Нептун)
        {
            let orbital_distance = 30.069 * AU; 
            let r = SUN_RADIUS + orbital_distance; 
            let v = (G * SUN_MASS / r).sqrt();
            let mut n = scene.add_circle(3.88 * EARTH_RADIUS / 2.0); 
            let col = Rgba::new(0.2, 0.3, 0.9, 1.0); // Deep Blue
            n.set_color(col);
            n.set_position(Vec2::new(r, 0.0));
            Object {
                node: n,
                mass: 17.1 * MSUN / 333000.0, 
                velocity: Vec2::new(0.0, v),
                aclr: Vec2::ZERO,
                trail: Vec::new(),
                color: col,
            }
        },
    ];
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
                let  pos = all_circ[i].node.position();
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
                        ui.label(format!(
                            "{}: pos=({:.1},{:.1}) vel=({:.2},{:.2})",
                            names[i], pos.x, pos.y, vel.x, vel.y
                        ));
                    }
                    ui.horizontal(|ui| {
                        if ui.button("Enable trails").clicked() {
                            enable_trails = !enable_trails
                        }
                        if ui.button("Enable velocity and acceleration vector").clicked() {
                            enable_potencial = !enable_potencial
                        }
                    });
                });
        });
    }
}
