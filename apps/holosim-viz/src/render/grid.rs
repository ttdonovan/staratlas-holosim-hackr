use macroquad::prelude::*;
use nalgebra::Vector2;

use crate::camera::Camera2D;
use super::utils::transform_point;

pub fn draw_grid(cam_matrix: &Mat4, camera: &Camera2D) {
    // Calculate visible bounds
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    // Get world bounds visible on screen
    let top_left = camera.screen_to_world(Vector2::new(0.0, 0.0));
    let bottom_right = camera.screen_to_world(Vector2::new(screen_width, screen_height));
    
    // Grid spacing in world units (10.0 matches sector spacing)
    let grid_spacing = 10.0;
    
    // Calculate grid line alpha based on zoom level
    // More zoomed in = more opaque grid lines
    let alpha = (camera.zoom * 30.0).min(60.0) as u8;
    let grid_color = Color::from_rgba(100, 100, 100, alpha);
    let major_grid_color = Color::from_rgba(150, 150, 150, (alpha as f32 * 1.5).min(90.0) as u8);
    
    // Round to nearest grid line
    let start_x = ((top_left.x / grid_spacing).floor() * grid_spacing) as i32;
    let end_x = ((bottom_right.x / grid_spacing).ceil() * grid_spacing) as i32;
    let start_y = ((bottom_right.y / grid_spacing).floor() * grid_spacing) as i32;
    let end_y = ((top_left.y / grid_spacing).ceil() * grid_spacing) as i32;
    
    // Draw vertical lines
    for x in (start_x..=end_x).step_by(grid_spacing as usize) {
        let world_start = Vector2::new(x as f32, start_y as f32);
        let world_end = Vector2::new(x as f32, end_y as f32);
        
        let screen_start = transform_point(cam_matrix, &world_start);
        let screen_end = transform_point(cam_matrix, &world_end);
        
        // Major grid lines every 5 sectors
        let color = if x % 50 == 0 {
            major_grid_color
        } else {
            grid_color
        };
        
        draw_line(
            screen_start.x, screen_start.y,
            screen_end.x, screen_end.y,
            1.0,
            color
        );
    }
    
    // Draw horizontal lines
    for y in (start_y..=end_y).step_by(grid_spacing as usize) {
        let world_start = Vector2::new(start_x as f32, y as f32);
        let world_end = Vector2::new(end_x as f32, y as f32);
        
        let screen_start = transform_point(cam_matrix, &world_start);
        let screen_end = transform_point(cam_matrix, &world_end);
        
        // Major grid lines every 5 sectors
        let color = if y % 50 == 0 {
            major_grid_color
        } else {
            grid_color
        };
        
        draw_line(
            screen_start.x, screen_start.y,
            screen_end.x, screen_end.y,
            1.0,
            color
        );
    }
}