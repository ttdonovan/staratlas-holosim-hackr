use macroquad::prelude::*;

use crate::camera::Camera2D;
use crate::ui::UIState;

pub fn handle_input(camera: &mut Camera2D, ui_state: &mut UIState) {
    // Camera movement with arrow keys or WASD
    let move_speed = 5.0 * (1.0 / camera.zoom);
    
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        camera.position.x -= move_speed;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        camera.position.x += move_speed;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        camera.position.y += move_speed;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        camera.position.y -= move_speed;
    }
    
    // Zoom with mouse wheel
    let wheel = mouse_wheel();
    if wheel.1 != 0.0 {
        camera.zoom *= 1.0 + wheel.1 * 0.1;
        camera.zoom = camera.zoom.clamp(0.01, 10.0);
    }
    
    // Toggle UI panels
    if is_key_pressed(KeyCode::F1) {
        ui_state.show_info_panel = !ui_state.show_info_panel;
    }
    if is_key_pressed(KeyCode::F2) {
        ui_state.show_search = !ui_state.show_search;
    }
    if is_key_pressed(KeyCode::G) {
        ui_state.show_grid = !ui_state.show_grid;
    }
    
    // Mouse hover detection
    let mouse_pos = mouse_position();
    let world_pos = camera.screen_to_world(nalgebra::Vector2::new(mouse_pos.0, mouse_pos.1));
    ui_state.mouse_world_pos = world_pos;
}