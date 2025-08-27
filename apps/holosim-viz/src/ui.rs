use nalgebra::Vector2;
use ui_holosim::{StarbaseUI, SectorUI};

#[derive(Clone, Debug)]
pub enum PinnedItem {
    Starbase(StarbaseUI),
    Sector(SectorUI),
}

pub struct UIState {
    pub show_info_panel: bool,
    pub show_search: bool,
    pub show_grid: bool,
    pub mouse_world_pos: Vector2<f32>,
    pub hovered_starbase: Option<StarbaseUI>,
    pub pinned_item: Option<PinnedItem>,
    pub pinned_position: Option<(f32, f32)>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            show_info_panel: true,
            show_search: false,
            show_grid: false,
            mouse_world_pos: Vector2::new(0.0, 0.0),
            hovered_starbase: None,
            pinned_item: None,
            pinned_position: None,
        }
    }
}