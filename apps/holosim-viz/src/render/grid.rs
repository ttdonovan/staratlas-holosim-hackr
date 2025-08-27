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
    let bold_grid_color = Color::from_rgba(200, 200, 100, (alpha as f32 * 2.0).min(120.0) as u8);
    
    // Offset by half a grid cell so sectors are centered in cells
    // Grid lines should be at -5, 5, 15, etc. so sectors at 0, 10, 20 are centered
    let offset = grid_spacing / 2.0;
    
    // Round to nearest grid line
    let start_x = (((top_left.x + offset) / grid_spacing).floor() * grid_spacing - offset) as i32;
    let end_x = (((bottom_right.x + offset) / grid_spacing).ceil() * grid_spacing - offset) as i32;
    let start_y = (((bottom_right.y + offset) / grid_spacing).floor() * grid_spacing - offset) as i32;
    let end_y = (((top_left.y + offset) / grid_spacing).ceil() * grid_spacing - offset) as i32;
    
    // Draw vertical lines
    for x in (start_x..=end_x).step_by(grid_spacing as usize) {
        let world_start = Vector2::new(x as f32, start_y as f32);
        let world_end = Vector2::new(x as f32, end_y as f32);
        
        let screen_start = transform_point(cam_matrix, &world_start);
        let screen_end = transform_point(cam_matrix, &world_end);
        
        // Determine line color and thickness based on position
        // x + offset gives us the actual sector coordinate this line is adjacent to
        let sector_x = (x as f32 + offset) as i32;
        let (color, thickness) = if sector_x % 100 == 0 && sector_x != 0 {
            (bold_grid_color, 2.0) // Bold every 10 sectors
        } else if sector_x % 50 == 0 {
            (major_grid_color, 1.5) // Major every 5 sectors
        } else {
            (grid_color, 1.0) // Regular grid
        };
        
        draw_line(
            screen_start.x, screen_start.y,
            screen_end.x, screen_end.y,
            thickness,
            color
        );
    }
    
    // Draw horizontal lines
    for y in (start_y..=end_y).step_by(grid_spacing as usize) {
        let world_start = Vector2::new(start_x as f32, y as f32);
        let world_end = Vector2::new(end_x as f32, y as f32);
        
        let screen_start = transform_point(cam_matrix, &world_start);
        let screen_end = transform_point(cam_matrix, &world_end);
        
        // Determine line color and thickness based on position
        // y + offset gives us the actual sector coordinate this line is adjacent to
        let sector_y = (y as f32 + offset) as i32;
        let (color, thickness) = if sector_y % 100 == 0 && sector_y != 0 {
            (bold_grid_color, 2.0) // Bold every 10 sectors
        } else if sector_y % 50 == 0 {
            (major_grid_color, 1.5) // Major every 5 sectors
        } else {
            (grid_color, 1.0) // Regular grid
        };
        
        draw_line(
            screen_start.x, screen_start.y,
            screen_end.x, screen_end.y,
            thickness,
            color
        );
    }
    
    // Draw special lines at origin (0,0)
    // Vertical line through x=5 (adjacent to x=0 sector)
    let origin_x = -offset;
    if origin_x >= start_x as f32 && origin_x <= end_x as f32 {
        let world_start = Vector2::new(origin_x, start_y as f32);
        let world_end = Vector2::new(origin_x, end_y as f32);
        let screen_start = transform_point(cam_matrix, &world_start);
        let screen_end = transform_point(cam_matrix, &world_end);
        
        draw_line(
            screen_start.x, screen_start.y,
            screen_end.x, screen_end.y,
            2.0,
            Color::from_rgba(255, 255, 100, (alpha as f32 * 2.0).min(150.0) as u8)
        );
    }
    
    // Horizontal line through y=5 (adjacent to y=0 sector)
    let origin_y = -offset;
    if origin_y >= start_y as f32 && origin_y <= end_y as f32 {
        let world_start = Vector2::new(start_x as f32, origin_y);
        let world_end = Vector2::new(end_x as f32, origin_y);
        let screen_start = transform_point(cam_matrix, &world_start);
        let screen_end = transform_point(cam_matrix, &world_end);
        
        draw_line(
            screen_start.x, screen_start.y,
            screen_end.x, screen_end.y,
            2.0,
            Color::from_rgba(255, 255, 100, (alpha as f32 * 2.0).min(150.0) as u8)
        );
    }
}