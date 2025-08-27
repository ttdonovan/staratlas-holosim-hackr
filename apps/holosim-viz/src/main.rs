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
        
        // Draw starbases
        draw_starbases(&game_data, &sector_positions, &cam_matrix);
        
        // Draw UI overlay
        draw_ui(&game_data, &ui_state, &camera);
        
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
) {
    for starbase in &game_data.starbases {
        if let Some(pos) = sector_positions.get(&starbase.sector) {
            let screen_pos = transform_point(cam_matrix, pos);
            
            // Draw starbase icon (larger than sector dot)
            let color = match starbase.faction {
                0 => Color::from_rgba(100, 255, 100, 255), // MUD faction - green
                1 => Color::from_rgba(255, 100, 100, 255), // ONI faction - red
                2 => Color::from_rgba(100, 100, 255, 255), // Ustur faction - blue
                _ => WHITE,
            };
            
            draw_circle(screen_pos.x, screen_pos.y, 6.0, color);
            draw_circle_lines(screen_pos.x, screen_pos.y, 8.0, 2.0, color);
        }
    }
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