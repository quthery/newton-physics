use kiss3d::prelude::*;
use crate::state::{GameState};

const TRAIL_RECORD_INTERVAL: usize = 5;
const MAX_TRAIL_LENGTH: usize = 500;

pub fn ui(state: &mut GameState) {
    state.window.draw_ui(|ctx: &egui::Context| {
        egui::Window::new("DRAG ON THE COLESIKO MISHI")
            .default_width(300.0)
            .show(ctx, |ui| {
                let names = [
                    "Sun", "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus",
                    "Neptune",
                ];
                for (i, obj) in state.objs.iter().enumerate() {
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
                        state.enable_trails = !state.enable_trails
                    }
                    if ui
                        .button("Enable velocity and acceleration vector")
                        .clicked()
                    {
                        state.enable_potencial = !state.enable_potencial
                    }
                });
            });
    });
}

pub fn potencial(state: &mut GameState) {
    for obj in state.objs.iter() {
        let pos = obj.node.position();
        state.window.draw_line_2d(pos, pos + obj.aclr * 1000.0, Rgba::new(1.0, 0.2, 0.2, 1.0), 3.0);
        state.window.draw_line_2d(pos, pos + obj.velocity * 50.0, Rgba::new(0.2, 1.0, 0.2, 1.0), 3.0);
        let dir = obj.velocity.normalize();
        let perp = Vec2::new(-dir.y, dir.x);
        let end = pos + obj.velocity * 50.0;
        let head_len = 10.0;
        let head_width = 5.0;
        state.window.draw_line_2d(end, end - dir * head_len + perp * head_width, GREEN, 10.0);
        state.window.draw_line_2d(end, end - dir * head_len - perp * head_width, GREEN, 10.0);
    }
}

pub fn trails(state: &mut GameState) {
    if state.frame_count % TRAIL_RECORD_INTERVAL as i32 == 0 {
        for obj in state.objs.iter_mut() {
            obj.trail.push(obj.node.position());
            if obj.trail.len() > MAX_TRAIL_LENGTH {
                obj.trail.remove(0);
            }
        }
    }
    state.frame_count += 1;

    for obj in state.objs.iter() {
        if obj.trail.len() > 1 {
            for i in 0..obj.trail.len() - 1 {
                state.window.draw_line_2d(obj.trail[i], obj.trail[i + 1], obj.color, 2.0);
            }
        }
    }
}
