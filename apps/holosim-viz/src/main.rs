use macroquad::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector2;

mod data;
mod camera;
mod ui;
mod render;
mod input;

use data::GameData;
use camera::Camera2D;
use ui::UIState;
use input::handle_input;
use render::{draw_sectors, draw_starbases, draw_ui, draw_minimap, draw_starbase_modal, draw_sector_modal, draw_grid, draw_origin_marker, handle_click};

#[macroquad::main("Star Atlas Galaxy Map")]
async fn main() {
    // Load game data
    let game_data_path = if std::path::Path::new("gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron").exists() {
        "gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron"
    } else {
        "../../gamedata/GAMEC7U7cqmFFaRow33j1LwuV8u4YhAS1mJ5Dqjnar2k/game_balance.ron"
    };
    let game_data = match GameData::load_from_file(game_data_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to load game data: {}", e);
            return;
        }
    };

    // Initialize camera centered on (0, 0)
    let mut camera = Camera2D::new();
    
    // UI state
    let mut ui_state = UIState::new();
    
    // Precompute sector positions for efficient rendering
    let sector_positions: HashMap<(i64, i64), Vector2<f32>> = game_data.sectors
        .iter()
        .map(|sector| {
            let pos = Vector2::new(
                sector.coordinates.0 as f32 * 10.0,
                sector.coordinates.1 as f32 * 10.0,
            );
            (sector.coordinates, pos)
        })
        .collect();
    
    // Debug: Check sector distribution
    eprintln!("\n=== Sector Data Analysis ===");
    eprintln!("Total sectors loaded: {}", game_data.sectors.len());
    
    // Find bounds of sectors
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    
    for sector in &game_data.sectors {
        min_x = min_x.min(sector.coordinates.0);
        max_x = max_x.max(sector.coordinates.0);
        min_y = min_y.min(sector.coordinates.1);
        max_y = max_y.max(sector.coordinates.1);
    }
    
    eprintln!("Sector bounds: X[{}, {}], Y[{}, {}]", min_x, max_x, min_y, max_y);
    
    // Calculate expected sectors vs actual
    let expected_sectors = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize;
    eprintln!("Expected sectors in rectangle: {}", expected_sectors);
    eprintln!("Actual sectors: {}", game_data.sectors.len());
    eprintln!("Coverage: {:.1}%", (game_data.sectors.len() as f32 / expected_sectors as f32) * 100.0);
    
    // Check for gaps in a small area
    eprintln!("\nChecking sector gaps around origin (-5 to 5):");
    for y in -5..=5 {
        for x in -5..=5 {
            if !sector_positions.contains_key(&(x, y)) {
                eprintln!("  Missing sector at ({}, {})", x, y);
            }
        }
    }
    
    // Create mine item lookup table (pubkey -> name)
    // We need to map from MineItem account pubkeys to names
    let mine_item_names: HashMap<String, String> = game_data.mine_items
        .iter()
        .filter_map(|item| {
            item.pubkey.as_ref().map(|pubkey| (pubkey.clone(), item.name.clone()))
        })
        .collect();
    
    eprintln!("Created mine_item_names lookup with {} entries", mine_item_names.len());
    
    // Create resource lookup table (location -> Vec<mine_item>)
    let mut resource_locations: HashMap<String, Vec<String>> = HashMap::new();
    for resource in &game_data.resources {
        resource_locations
            .entry(resource.location.clone())
            .or_insert_with(Vec::new)
            .push(resource.mine_item.clone());
    }
    
    eprintln!("\n=== Resource System Initialization ===");
    eprintln!("Total resources: {}", game_data.resources.len());
    eprintln!("Total mine items: {}", game_data.mine_items.len());
    eprintln!("Resource locations map created with {} entries", resource_locations.len());
    
    // Log first few resources for debugging
    for (i, resource) in game_data.resources.iter().take(5).enumerate() {
        eprintln!("Resource {}: location={}, mine_item={}", i, resource.location, resource.mine_item);
    }
    
    // Log mine items
    eprintln!("\nMine items:");
    for item in &game_data.mine_items {
        eprintln!("  {} -> {}", item.mint, item.name);
    }

    // Main game loop
    loop {
        // Handle input
        handle_input(&mut camera, &mut ui_state);
        
        // Handle clicks
        handle_click(&game_data, &sector_positions, &camera, &mut ui_state);
        
        // Clear background
        clear_background(Color::from_rgba(10, 10, 25, 255));
        
        // Apply camera transform
        let cam_matrix = camera.matrix();
        
        // Draw grid if enabled
        if ui_state.show_grid {
            draw_grid(&cam_matrix, &camera);
            draw_origin_marker(&cam_matrix, &camera);
        }
        
        // Draw sectors
        draw_sectors(&game_data, &sector_positions, &cam_matrix, &ui_state);
        
        // Draw starbases and check for hover
        draw_starbases(&game_data, &sector_positions, &cam_matrix, &mut ui_state);
        
        // Draw UI overlay
        draw_ui(&game_data, &ui_state, &camera);
        
        // Draw starbase modal if hovering or pinned
        draw_starbase_modal(&ui_state, &game_data, &mine_item_names, &resource_locations, &camera);
        
        // Draw sector modal if pinned
        draw_sector_modal(&ui_state, &game_data, &camera);
        
        // Draw minimap
        draw_minimap(&sector_positions, &camera);
        
        next_frame().await;
    }
}