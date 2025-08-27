use macroquad::prelude::*;

use crate::ui::{UIState, PinnedItem};
use crate::camera::Camera2D;
use crate::data::GameData;

pub fn draw_sector_modal(
    ui_state: &UIState,
    game_data: &GameData,
    camera: &Camera2D
) {
    // Only show for pinned sectors (not hovered)
    if let Some(PinnedItem::Sector(sector)) = &ui_state.pinned_item {
        
        // Scale modal size based on zoom level
        let zoom_scale = camera.zoom.max(1.0).min(3.0);
        let modal_width = 280.0 * zoom_scale;
        let modal_height = 250.0 * zoom_scale;
        let base_font_size = 16.0 * zoom_scale;
        let title_font_size = 20.0 * zoom_scale;
        let _small_font_size = 14.0 * zoom_scale;
        let line_height = 22.0 * zoom_scale;
        
        // Position modal at pinned position
        let mouse_pos = ui_state.pinned_position.unwrap_or((screen_width() / 2.0, screen_height() / 2.0));
        let mut modal_x = mouse_pos.0 + 20.0;
        let mut modal_y = mouse_pos.1 + 20.0;
        
        // Keep modal on screen
        if modal_x + modal_width > screen_width() {
            modal_x = modal_x.max(20.0) - modal_width - 40.0;
        }
        if modal_y + modal_height > screen_height() {
            modal_y = modal_y.max(20.0) - modal_height - 40.0;
        }
        
        // Draw modal background
        draw_rectangle(modal_x, modal_y, modal_width, modal_height, Color::from_rgba(20, 20, 30, 240));
        draw_rectangle_lines(modal_x, modal_y, modal_width, modal_height, 2.0, WHITE);
        
        // Draw header
        draw_rectangle(modal_x, modal_y, modal_width, 40.0 * zoom_scale, Color::from_rgba(50, 50, 70, 240));
        
        // Title
        draw_text(&sector.name, modal_x + 10.0, modal_y + 25.0 * zoom_scale, title_font_size, WHITE);
        
        let mut y_offset = modal_y + 60.0 * zoom_scale;
        
        // Coordinates
        draw_text("Coordinates:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("({}, {})", sector.coordinates.0, sector.coordinates.1), modal_x + 120.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Stars
        draw_text("Stars:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}", sector.num_stars), modal_x + 120.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Planets
        draw_text("Planets:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}", sector.num_planets), modal_x + 120.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Check if there's a starbase in this sector
        let has_starbase = game_data.starbases.iter().any(|sb| sb.sector == sector.coordinates);
        if has_starbase {
            y_offset += 10.0 * zoom_scale;
            draw_text("⚠️ Has Starbase", modal_x + 10.0, y_offset, base_font_size, Color::from_rgba(255, 220, 100, 255));
        }
        
        // Close instruction
        draw_text("Press ESC to close", modal_x + modal_width - 150.0 * zoom_scale, modal_y + modal_height - 25.0 * zoom_scale, _small_font_size, Color::from_rgba(150, 150, 150, 255));
        
    }
}