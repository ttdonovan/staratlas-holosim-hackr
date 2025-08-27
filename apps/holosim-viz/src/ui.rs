use nalgebra::Vector2;
use ui_holosim::StarbaseUI;

pub struct UIState {
    pub show_info_panel: bool,
    pub show_search: bool,
    pub show_grid: bool,
    pub mouse_world_pos: Vector2<f32>,
    pub hovered_starbase: Option<StarbaseUI>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            show_info_panel: true,
            show_search: false,
            show_grid: false,
            mouse_world_pos: Vector2::new(0.0, 0.0),
            hovered_starbase: None,
        }
    }
}