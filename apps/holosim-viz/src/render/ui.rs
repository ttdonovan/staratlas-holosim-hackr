use macroquad::prelude::*;

use crate::data::GameData;
use crate::ui::UIState;
use crate::camera::Camera2D;

pub fn draw_ui(game_data: &GameData, ui_state: &UIState, camera: &Camera2D) {
    // Info panel
    if ui_state.show_info_panel {
        let panel_width = 300.0;
        let panel_height = 200.0;
        
        draw_rectangle(10.0, 10.0, panel_width, panel_height, Color::from_rgba(0, 0, 0, 200));
        draw_rectangle_lines(10.0, 10.0, panel_width, panel_height, 2.0, WHITE);
        
        draw_text("Star Atlas Galaxy Map", 20.0, 30.0, 20.0, WHITE);
        draw_text(&format!("Total Sectors: {}", game_data.sectors.len()), 20.0, 50.0, 16.0, WHITE);
        draw_text(&format!("Total Ships: {}", game_data.ships.len()), 20.0, 70.0, 16.0, WHITE);
        draw_text(&format!("Total Planets: {}", game_data.planets.len()), 20.0, 90.0, 16.0, WHITE);
        draw_text(&format!("Total Starbases: {}", game_data.starbases.len()), 20.0, 110.0, 16.0, WHITE);
        
        draw_text(&format!("Camera: ({:.1}, {:.1})", camera.position.x, camera.position.y), 20.0, 140.0, 14.0, GRAY);
        draw_text(&format!("Zoom: {:.2}x", camera.zoom), 20.0, 160.0, 14.0, GRAY);
        
        draw_text("F1: Info | F2: Search | G: Grid | Scroll: Zoom", 20.0, 190.0, 12.0, GRAY);
    }
}