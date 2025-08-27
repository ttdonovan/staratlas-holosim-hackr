use macroquad::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector2;

use crate::data::GameData;
use crate::ui::{UIState, PinnedItem};
use crate::camera::Camera2D;
use super::utils::transform_point;

pub fn draw_starbases(
    game_data: &GameData,
    sector_positions: &HashMap<(i64, i64), Vector2<f32>>,
    cam_matrix: &Mat4,
    ui_state: &mut UIState,
) {
    let mut hovered_starbase = None;
    let hover_distance = 15.0; // pixels
    
    for starbase in &game_data.starbases {
        if let Some(pos) = sector_positions.get(&starbase.sector) {
            let screen_pos = transform_point(cam_matrix, pos);
            
            // Check if mouse is hovering over this starbase
            let mouse_pos = mouse_position();
            let distance = ((mouse_pos.0 - screen_pos.x).powi(2) + (mouse_pos.1 - screen_pos.y).powi(2)).sqrt();
            
            let is_hovered = distance < hover_distance;
            if is_hovered {
                hovered_starbase = Some(starbase.clone());
            }
            
            // Draw starbase icon (larger than sector dot)
            // Determine faction color based on starbase name prefix
            let base_color = if starbase.name.starts_with("MUD") {
                Color::from_rgba(255, 100, 100, 255) // MUD faction - red
            } else if starbase.name.starts_with("ONI") {
                Color::from_rgba(100, 100, 255, 255) // ONI faction - blue
            } else if starbase.name.starts_with("Ustur") {
                Color::from_rgba(255, 220, 100, 255) // Ustur faction - yellow
            } else {
                WHITE
            };
            
            // Check if this starbase is pinned
            let is_pinned = matches!(
                &ui_state.pinned_item,
                Some(PinnedItem::Starbase(s)) if s.seq_id == starbase.seq_id
            );
            
            // Highlight if hovered or pinned
            let color = if is_hovered {
                Color::from_rgba(
                    (base_color.r * 255.0 * 1.3).min(255.0) as u8,
                    (base_color.g * 255.0 * 1.3).min(255.0) as u8,
                    (base_color.b * 255.0 * 1.3).min(255.0) as u8,
                    255
                )
            } else if is_pinned {
                Color::from_rgba(
                    (base_color.r * 255.0 * 1.2).min(255.0) as u8,
                    (base_color.g * 255.0 * 1.2).min(255.0) as u8,
                    (base_color.b * 255.0 * 1.2).min(255.0) as u8,
                    255
                )
            } else {
                base_color
            };
            
            let size = if is_hovered || is_pinned { 8.0 } else { 6.0 };
            draw_circle(screen_pos.x, screen_pos.y, size, color);
            draw_circle_lines(screen_pos.x, screen_pos.y, size + 2.0, 2.0, color);
            
            // Draw pin indicator if pinned
            if is_pinned {
                draw_circle_lines(screen_pos.x, screen_pos.y, size + 4.0, 1.0, Color::from_rgba(255, 255, 150, 200));
            }
        }
    }
    
    ui_state.hovered_starbase = hovered_starbase;
}

pub fn draw_starbase_modal(
    ui_state: &UIState, 
    game_data: &GameData, 
    mine_item_names: &HashMap<String, String>,
    resource_locations: &HashMap<String, Vec<String>>,
    camera: &Camera2D
) {
    // Show modal for either hovered starbase or pinned starbase
    let starbase = if let Some(starbase) = &ui_state.hovered_starbase {
        Some(starbase)
    } else if let Some(PinnedItem::Starbase(starbase)) = &ui_state.pinned_item {
        Some(starbase)
    } else {
        None
    };
    
    if let Some(starbase) = starbase {
        // Check if this is pinned
        let is_pinned = matches!(
            &ui_state.pinned_item,
            Some(PinnedItem::Starbase(s)) if s.seq_id == starbase.seq_id
        );
        let mouse_pos = mouse_position();
        
        // Scale modal size based on zoom level
        // When zoomed in (zoom > 1), make modal larger
        // When zoomed out (zoom < 1), use default size
        let zoom_scale = camera.zoom.max(1.0).min(3.0); // Cap at 3x size
        let modal_width = 320.0 * zoom_scale;
        let modal_height = 500.0 * zoom_scale;
        let base_font_size = 16.0 * zoom_scale;
        let title_font_size = 22.0 * zoom_scale;
        let small_font_size = 14.0 * zoom_scale;
        let line_height = 22.0 * zoom_scale;
        
        // Position modal near mouse but ensure it stays on screen
        let mut modal_x = mouse_pos.0 + 20.0;
        let mut modal_y = mouse_pos.1 + 20.0;
        
        // Keep modal on screen
        if modal_x + modal_width > screen_width() {
            modal_x = mouse_pos.0 - modal_width - 20.0;
        }
        if modal_y + modal_height > screen_height() {
            modal_y = mouse_pos.1 - modal_height - 20.0;
        }
        
        // Draw modal background
        draw_rectangle(modal_x, modal_y, modal_width, modal_height, Color::from_rgba(20, 20, 30, 240));
        draw_rectangle_lines(modal_x, modal_y, modal_width, modal_height, 2.0, WHITE);
        
        // Get faction name and color based on starbase name prefix
        let (faction_name, faction_color) = if starbase.name.starts_with("MUD") {
            ("MUD", Color::from_rgba(255, 100, 100, 255))     // Red
        } else if starbase.name.starts_with("ONI") {
            ("ONI", Color::from_rgba(100, 100, 255, 255))     // Blue
        } else if starbase.name.starts_with("Ustur") {
            ("Ustur", Color::from_rgba(255, 220, 100, 255))   // Yellow
        } else {
            ("Unknown", WHITE)
        };
        
        // Draw header with faction color
        draw_rectangle(modal_x, modal_y, modal_width, 40.0 * zoom_scale, Color::from_rgba(
            (faction_color.r * 255.0 * 0.3) as u8,
            (faction_color.g * 255.0 * 0.3) as u8,
            (faction_color.b * 255.0 * 0.3) as u8,
            240
        ));
        
        // Title
        draw_text(&starbase.name, modal_x + 10.0, modal_y + 25.0 * zoom_scale, title_font_size, WHITE);
        
        // Pin indicator
        if is_pinned {
            draw_text("📍", modal_x + modal_width - 40.0 * zoom_scale, modal_y + 25.0 * zoom_scale, title_font_size, Color::from_rgba(255, 255, 150, 255));
        }
        
        // Close button hint
        draw_text("ESC to close", modal_x + modal_width - 90.0 * zoom_scale, modal_y + modal_height - 20.0 * zoom_scale, small_font_size * 0.8, Color::from_rgba(150, 150, 150, 200));
        
        let mut y_offset = modal_y + 60.0 * zoom_scale;
        
        // Faction
        draw_text("Faction:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(faction_name, modal_x + 100.0 * zoom_scale, y_offset, base_font_size, faction_color);
        y_offset += line_height;
        
        // Sector coordinates
        draw_text("Sector:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("({}, {})", starbase.sector.0, starbase.sector.1), modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Level
        draw_text("Level:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}", starbase.level), modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // State
        let state_text = match starbase.state {
            0 => "Active",
            1 => "Under Construction",
            2 => "Upgrading",
            _ => "Unknown",
        };
        draw_text("State:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(state_text, modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // HP/SP
        draw_text("HP/SP:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}/{}", starbase.hp, starbase.sp), modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Draw separator
        y_offset += 10.0 * zoom_scale;
        draw_line(modal_x + 10.0, y_offset, modal_x + modal_width - 10.0, y_offset, 1.0 * zoom_scale, Color::from_rgba(100, 100, 100, 100));
        y_offset += 15.0 * zoom_scale;
        
        // Upkeep Resources
        draw_text("Upkeep Resources", modal_x + 10.0, y_offset, base_font_size * 1.125, WHITE);
        y_offset += line_height;
        
        // Ammo
        draw_text("Ammo:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}", starbase.upkeep_ammo_balance), modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Food
        draw_text("Food:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}", starbase.upkeep_food_balance), modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Toolkit
        draw_text("Toolkit:", modal_x + 10.0, y_offset, base_font_size, GRAY);
        draw_text(&format!("{}", starbase.upkeep_toolkit_balance), modal_x + 100.0 * zoom_scale, y_offset, base_font_size, WHITE);
        y_offset += line_height;
        
        // Draw separator
        y_offset += 10.0 * zoom_scale;
        draw_line(modal_x + 10.0, y_offset, modal_x + modal_width - 10.0, y_offset, 1.0 * zoom_scale, Color::from_rgba(100, 100, 100, 100));
        y_offset += 15.0 * zoom_scale;
        
        // Find planets in the same sector
        let sector_planets: Vec<&ui_holosim::PlanetUI> = game_data.planets.iter()
            .filter(|planet| planet.sector[0] == starbase.sector.0 && planet.sector[1] == starbase.sector.1)
            .collect();
        
        // Show sector resources
        draw_text("Sector Info", modal_x + 10.0, y_offset, base_font_size * 1.125, WHITE);
        y_offset += line_height;
        
        if sector_planets.is_empty() {
            draw_text("No planets in sector", modal_x + 10.0, y_offset, small_font_size, Color::from_rgba(150, 150, 150, 255));
        } else {
            let total_resources: u8 = sector_planets.iter().map(|p| p.num_resources).sum();
            draw_text(&format!("Planets: {}", sector_planets.len()), modal_x + 10.0, y_offset, base_font_size, GRAY);
            draw_text(&format!("Resources: {}", total_resources), modal_x + 150.0 * zoom_scale, y_offset, base_font_size, WHITE);
            y_offset += line_height;
            
            // Find mineable resources in this sector
            let mut sector_resources: Vec<String> = Vec::new();
            
            // Debug logging
            eprintln!("\n=== Starbase: {} at sector {:?} ===", starbase.name, starbase.sector);
            eprintln!("Found {} planets in sector", sector_planets.len());
            
            for planet in &sector_planets {
                eprintln!("  Planet: {} - pubkey: {:?}", planet.name, planet.pubkey);
                
                if let Some(planet_pubkey) = &planet.pubkey {
                    // Check if any resources have this planet as their location
                    if let Some(mine_item_pubkeys) = resource_locations.get(planet_pubkey) {
                        eprintln!("    Found {} resources at this planet!", mine_item_pubkeys.len());
                        
                        for mine_item_pubkey in mine_item_pubkeys {
                            eprintln!("      Resource mine_item pubkey: {}", mine_item_pubkey);
                            
                            if let Some(resource_name) = mine_item_names.get(mine_item_pubkey) {
                                eprintln!("      Resource name: {}", resource_name);
                                if !sector_resources.contains(resource_name) {
                                    sector_resources.push(resource_name.clone());
                                }
                            } else {
                                eprintln!("      WARNING: No name found for mine_item pubkey: {}", mine_item_pubkey);
                            }
                        }
                    } else {
                        eprintln!("    No resources found at planet pubkey: {}", planet_pubkey);
                    }
                } else {
                    eprintln!("    Planet has no pubkey!");
                }
            }
            
            eprintln!("Total unique resources found: {}", sector_resources.len());
            eprintln!("Resource locations map size: {}", resource_locations.len());
            eprintln!("Mine item names map size: {}", mine_item_names.len());
            
            if !sector_resources.is_empty() {
                draw_text("Mineable Resources:", modal_x + 10.0, y_offset, small_font_size, GRAY);
                y_offset += 18.0 * zoom_scale;
                
                sector_resources.sort();
                for (i, name) in sector_resources.iter().take(5).enumerate() {
                    draw_text(&format!("• {}", name), modal_x + 20.0 * zoom_scale, y_offset + (i as f32 * base_font_size), small_font_size * 0.857, Color::from_rgba(100, 255, 100, 255));
                }
                if sector_resources.len() > 5 {
                    draw_text(&format!("... and {} more", sector_resources.len() - 5), modal_x + 20.0 * zoom_scale, y_offset + 80.0 * zoom_scale, small_font_size * 0.857, Color::from_rgba(150, 150, 150, 255));
                }
            } else if total_resources > 0 {
                draw_text("Resources not yet mapped", modal_x + 10.0, y_offset, small_font_size, Color::from_rgba(150, 150, 150, 255));
            }
        }
        
        // Sequence ID (bottom)
        draw_text(&format!("ID: {}", starbase.seq_id), modal_x + 10.0, modal_y + modal_height - 25.0 * zoom_scale, small_font_size, Color::from_rgba(150, 150, 150, 255));
    }
}