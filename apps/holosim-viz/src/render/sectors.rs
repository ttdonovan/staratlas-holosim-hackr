use macroquad::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector2;

use crate::data::GameData;
use crate::ui::UIState;
use super::utils::transform_point;

pub fn draw_sectors(
    game_data: &GameData,
    sector_positions: &HashMap<(i64, i64), Vector2<f32>>,
    cam_matrix: &Mat4,
    ui_state: &UIState,
) {
    // Get screen bounds for culling
    let screen_width = screen_width();
    let screen_height = screen_height();
    
    for sector in game_data.sectors.iter() {
        if let Some(pos) = sector_positions.get(&sector.coordinates) {
            // Transform to screen space
            let screen_pos = transform_point(cam_matrix, pos);
            
            // Cull sectors outside screen
            if screen_pos.x < -50.0 || screen_pos.x > screen_width + 50.0 ||
               screen_pos.y < -50.0 || screen_pos.y > screen_height + 50.0 {
                continue;
            }
            
            // Determine color based on properties
            let color = if sector.num_stars > 0 {
                Color::from_rgba(255, 220, 100, 200) // Yellow for sectors with stars
            } else if sector.num_planets > 0 {
                Color::from_rgba(100, 200, 255, 150) // Blue for sectors with planets
            } else {
                Color::from_rgba(100, 100, 100, 100) // Gray for empty sectors
            };
            
            // Draw sector dot
            draw_circle(screen_pos.x, screen_pos.y, 3.0, color);
            
            // Check if mouse is hovering over this sector
            let dist_sq = (ui_state.mouse_world_pos - pos).magnitude_squared();
            if dist_sq < 100.0 {
                // Draw sector info
                draw_text(
                    &format!("{} ({}, {})", sector.name, sector.coordinates.0, sector.coordinates.1),
                    screen_pos.x + 10.0,
                    screen_pos.y - 10.0,
                    16.0,
                    WHITE,
                );
            }
        }
    }
}