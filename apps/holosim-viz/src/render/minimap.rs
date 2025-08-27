use macroquad::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector2;

use crate::camera::Camera2D;

pub fn draw_minimap(sector_positions: &HashMap<(i64, i64), Vector2<f32>>, camera: &Camera2D) {
    let minimap_size = 150.0;
    let minimap_x = screen_width() - minimap_size - 10.0;
    let minimap_y = 10.0;
    
    // Draw minimap background
    draw_rectangle(minimap_x, minimap_y, minimap_size, minimap_size, Color::from_rgba(0, 0, 0, 200));
    draw_rectangle_lines(minimap_x, minimap_y, minimap_size, minimap_size, 2.0, WHITE);
    
    // Calculate bounds of all sectors
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    
    for pos in sector_positions.values() {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }
    
    let width = max_x - min_x;
    let height = max_y - min_y;
    let scale = (minimap_size - 10.0) / width.max(height);
    
    // Draw sectors on minimap
    for pos in sector_positions.values() {
        let x = minimap_x + 5.0 + (pos.x - min_x) * scale;
        let y = minimap_y + 5.0 + (pos.y - min_y) * scale;
        draw_circle(x, y, 1.0, Color::from_rgba(100, 100, 100, 200));
    }
    
    // Draw camera position
    let cam_x = minimap_x + 5.0 + (camera.position.x - min_x) * scale;
    let cam_y = minimap_y + 5.0 + (-camera.position.y - min_y) * scale;
    draw_rectangle_lines(cam_x - 5.0, cam_y - 5.0, 10.0, 10.0, 2.0, YELLOW);
}