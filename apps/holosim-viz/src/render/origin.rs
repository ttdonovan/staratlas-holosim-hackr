use macroquad::prelude::*;
use nalgebra::Vector2;

use crate::camera::Camera2D;
use super::utils::transform_point;

pub fn draw_origin_marker(cam_matrix: &Mat4, camera: &Camera2D) {
    // Draw a special marker at the origin (0,0)
    let origin = Vector2::new(0.0, 0.0);
    let screen_pos = transform_point(cam_matrix, &origin);
    
    // Only draw if on screen
    if screen_pos.x >= -50.0 && screen_pos.x <= screen_width() + 50.0 &&
       screen_pos.y >= -50.0 && screen_pos.y <= screen_height() + 50.0 {
        
        // Scale marker with zoom
        let size = (15.0 * camera.zoom).max(5.0).min(30.0);
        let alpha = ((camera.zoom * 100.0).min(200.0)) as u8;
        
        // Draw crosshair at origin
        let color = Color::from_rgba(255, 255, 100, alpha);
        draw_line(
            screen_pos.x - size, screen_pos.y,
            screen_pos.x + size, screen_pos.y,
            2.0,
            color
        );
        draw_line(
            screen_pos.x, screen_pos.y - size,
            screen_pos.x, screen_pos.y + size,
            2.0,
            color
        );
        
        // Draw circle
        draw_circle_lines(screen_pos.x, screen_pos.y, size * 0.5, 2.0, color);
        
        // Label if zoomed in enough
        if camera.zoom > 0.5 {
            draw_text(
                "Origin (0,0)",
                screen_pos.x + size + 5.0,
                screen_pos.y - 5.0,
                16.0 * camera.zoom.min(1.5),
                color
            );
        }
    }
}