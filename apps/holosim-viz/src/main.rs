use macroquad::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector2;

mod data;
mod camera;
mod ui;

use data::GameData;
use camera::Camera2D;
use ui::UIState;

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
        
        // Clear background
        clear_background(Color::from_rgba(10, 10, 25, 255));
        
        // Apply camera transform
        let cam_matrix = camera.matrix();
        
        // Draw sectors
        draw_sectors(&game_data, &sector_positions, &cam_matrix, &ui_state);
        
        // Draw starbases and check for hover
        draw_starbases(&game_data, &sector_positions, &cam_matrix, &mut ui_state);
        
        // Draw UI overlay
        draw_ui(&game_data, &ui_state, &camera);
        
        // Draw starbase modal if hovering
        draw_starbase_modal(&ui_state, &game_data, &mine_item_names, &resource_locations);
        
        // Draw minimap
        draw_minimap(&sector_positions, &camera);
        
        next_frame().await;
    }
}

fn handle_input(camera: &mut Camera2D, ui_state: &mut UIState) {
    // Camera movement with arrow keys or WASD
    let move_speed = 5.0 * (1.0 / camera.zoom);
    
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        camera.position.x -= move_speed;
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        camera.position.x += move_speed;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        camera.position.y += move_speed;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        camera.position.y -= move_speed;
    }
    
    // Zoom with mouse wheel
    let wheel = mouse_wheel();
    if wheel.1 != 0.0 {
        camera.zoom *= 1.0 + wheel.1 * 0.1;
        camera.zoom = camera.zoom.clamp(0.01, 10.0);
    }
    
    // Toggle UI panels
    if is_key_pressed(KeyCode::F1) {
        ui_state.show_info_panel = !ui_state.show_info_panel;
    }
    if is_key_pressed(KeyCode::F2) {
        ui_state.show_search = !ui_state.show_search;
    }
    
    // Mouse hover detection
    let mouse_pos = mouse_position();
    let world_pos = camera.screen_to_world(Vector2::new(mouse_pos.0, mouse_pos.1));
    ui_state.mouse_world_pos = world_pos;
}

fn draw_sectors(
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

fn draw_starbases(
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
            
            // Highlight if hovered
            let color = if is_hovered {
                Color::from_rgba(
                    (base_color.r * 255.0 * 1.3).min(255.0) as u8,
                    (base_color.g * 255.0 * 1.3).min(255.0) as u8,
                    (base_color.b * 255.0 * 1.3).min(255.0) as u8,
                    255
                )
            } else {
                base_color
            };
            
            let size = if is_hovered { 8.0 } else { 6.0 };
            draw_circle(screen_pos.x, screen_pos.y, size, color);
            draw_circle_lines(screen_pos.x, screen_pos.y, size + 2.0, 2.0, color);
        }
    }
    
    ui_state.hovered_starbase = hovered_starbase;
}

fn draw_ui(game_data: &GameData, ui_state: &UIState, camera: &Camera2D) {
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
        
        draw_text("F1: Toggle Info | F2: Search | Scroll: Zoom", 20.0, 190.0, 12.0, GRAY);
    }
}

fn draw_minimap(sector_positions: &HashMap<(i64, i64), Vector2<f32>>, camera: &Camera2D) {
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

fn transform_point(matrix: &Mat4, point: &Vector2<f32>) -> Vector2<f32> {
    let transformed = *matrix * vec4(point.x, -point.y, 0.0, 1.0);
    Vector2::new(transformed.x, transformed.y)
}

fn draw_starbase_modal(
    ui_state: &UIState, 
    game_data: &GameData, 
    mine_item_names: &HashMap<String, String>,
    resource_locations: &HashMap<String, Vec<String>>
) {
    if let Some(starbase) = &ui_state.hovered_starbase {
        let mouse_pos = mouse_position();
        let modal_width = 320.0;
        let modal_height = 500.0;
        
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
        draw_rectangle(modal_x, modal_y, modal_width, 40.0, Color::from_rgba(
            (faction_color.r * 255.0 * 0.3) as u8,
            (faction_color.g * 255.0 * 0.3) as u8,
            (faction_color.b * 255.0 * 0.3) as u8,
            240
        ));
        
        // Title
        draw_text(&starbase.name, modal_x + 10.0, modal_y + 25.0, 22.0, WHITE);
        
        let mut y_offset = modal_y + 60.0;
        let line_height = 22.0;
        
        // Faction
        draw_text("Faction:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(faction_name, modal_x + 100.0, y_offset, 16.0, faction_color);
        y_offset += line_height;
        
        // Sector coordinates
        draw_text("Sector:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(&format!("({}, {})", starbase.sector.0, starbase.sector.1), modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Level
        draw_text("Level:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(&format!("{}", starbase.level), modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // State
        let state_text = match starbase.state {
            0 => "Active",
            1 => "Under Construction",
            2 => "Upgrading",
            _ => "Unknown",
        };
        draw_text("State:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(state_text, modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // HP/SP
        draw_text("HP/SP:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(&format!("{}/{}", starbase.hp, starbase.sp), modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Draw separator
        y_offset += 10.0;
        draw_line(modal_x + 10.0, y_offset, modal_x + modal_width - 10.0, y_offset, 1.0, Color::from_rgba(100, 100, 100, 100));
        y_offset += 15.0;
        
        // Upkeep Resources
        draw_text("Upkeep Resources", modal_x + 10.0, y_offset, 18.0, WHITE);
        y_offset += line_height;
        
        // Ammo
        draw_text("Ammo:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(&format!("{}", starbase.upkeep_ammo_balance), modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Food
        draw_text("Food:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(&format!("{}", starbase.upkeep_food_balance), modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Toolkit
        draw_text("Toolkit:", modal_x + 10.0, y_offset, 16.0, GRAY);
        draw_text(&format!("{}", starbase.upkeep_toolkit_balance), modal_x + 100.0, y_offset, 16.0, WHITE);
        y_offset += line_height;
        
        // Draw separator
        y_offset += 10.0;
        draw_line(modal_x + 10.0, y_offset, modal_x + modal_width - 10.0, y_offset, 1.0, Color::from_rgba(100, 100, 100, 100));
        y_offset += 15.0;
        
        // Find planets in the same sector
        let sector_planets: Vec<&ui_holosim::PlanetUI> = game_data.planets.iter()
            .filter(|planet| planet.sector[0] == starbase.sector.0 && planet.sector[1] == starbase.sector.1)
            .collect();
        
        // Show sector resources
        draw_text("Sector Info", modal_x + 10.0, y_offset, 18.0, WHITE);
        y_offset += line_height;
        
        if sector_planets.is_empty() {
            draw_text("No planets in sector", modal_x + 10.0, y_offset, 14.0, Color::from_rgba(150, 150, 150, 255));
        } else {
            let total_resources: u8 = sector_planets.iter().map(|p| p.num_resources).sum();
            draw_text(&format!("Planets: {}", sector_planets.len()), modal_x + 10.0, y_offset, 16.0, GRAY);
            draw_text(&format!("Resources: {}", total_resources), modal_x + 150.0, y_offset, 16.0, WHITE);
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
                draw_text("Mineable Resources:", modal_x + 10.0, y_offset, 14.0, GRAY);
                y_offset += 18.0;
                
                sector_resources.sort();
                for (i, name) in sector_resources.iter().take(5).enumerate() {
                    draw_text(&format!("• {}", name), modal_x + 20.0, y_offset + (i as f32 * 16.0), 12.0, Color::from_rgba(100, 255, 100, 255));
                }
                if sector_resources.len() > 5 {
                    draw_text(&format!("... and {} more", sector_resources.len() - 5), modal_x + 20.0, y_offset + 80.0, 12.0, Color::from_rgba(150, 150, 150, 255));
                }
            } else if total_resources > 0 {
                draw_text("Resources not yet mapped", modal_x + 10.0, y_offset, 14.0, Color::from_rgba(150, 150, 150, 255));
            }
        }
        
        // Sequence ID (bottom)
        draw_text(&format!("ID: {}", starbase.seq_id), modal_x + 10.0, modal_y + modal_height - 25.0, 14.0, Color::from_rgba(150, 150, 150, 255));
    }
}