use macroquad::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector2;

use crate::data::GameData;
use crate::camera::Camera2D;
use crate::ui::{UIState, PinnedItem};

pub fn handle_click(
    game_data: &GameData,
    sector_positions: &HashMap<(i64, i64), Vector2<f32>>,
    camera: &Camera2D,
    ui_state: &mut UIState,
) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        let world_pos = camera.screen_to_world(Vector2::new(mouse_pos.0, mouse_pos.1));
        
        // First check if clicking on a starbase (they have priority)
        for starbase in &game_data.starbases {
            if let Some(pos) = sector_positions.get(&starbase.sector) {
                let distance = (world_pos - pos).magnitude();
                if distance < 10.0 { // Within clicking range
                    // Toggle pinning - if same starbase is clicked, unpin
                    if let Some(PinnedItem::Starbase(ref pinned)) = ui_state.pinned_item {
                        if pinned.seq_id == starbase.seq_id {
                            ui_state.pinned_item = None;
                            ui_state.pinned_position = None;
                            return;
                        }
                    }
                    ui_state.pinned_item = Some(PinnedItem::Starbase(starbase.clone()));
                    ui_state.pinned_position = Some(mouse_pos);
                    return;
                }
            }
        }
        
        // If not clicking on a starbase, check sectors
        for sector in &game_data.sectors {
            if let Some(pos) = sector_positions.get(&sector.coordinates) {
                let distance = (world_pos - pos).magnitude();
                if distance < 10.0 { // Within clicking range
                    // Toggle pinning - if same sector is clicked, unpin
                    if let Some(PinnedItem::Sector(ref pinned)) = ui_state.pinned_item {
                        if pinned.coordinates == sector.coordinates {
                            ui_state.pinned_item = None;
                            ui_state.pinned_position = None;
                            return;
                        }
                    }
                    ui_state.pinned_item = Some(PinnedItem::Sector(sector.clone()));
                    ui_state.pinned_position = Some(mouse_pos);
                    return;
                }
            }
        }
        
        // Clicked on empty space - clear pinned item
        ui_state.pinned_item = None;
        ui_state.pinned_position = None;
    }
    
    // Right click or Escape to clear pinned item
    if is_mouse_button_pressed(MouseButton::Right) || is_key_pressed(KeyCode::Escape) {
        ui_state.pinned_item = None;
        ui_state.pinned_position = None;
    }
}