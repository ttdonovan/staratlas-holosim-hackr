use macroquad::prelude::*;
use nalgebra::Vector2;

pub struct Camera2D {
    pub position: Vector2<f32>,
    pub zoom: f32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
    
    pub fn matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(vec3(
            screen_width() / 2.0 - self.position.x * self.zoom,
            screen_height() / 2.0 + self.position.y * self.zoom,
            0.0
        ));
        let scale = Mat4::from_scale(vec3(self.zoom, self.zoom, 1.0));
        translation * scale
    }
    
    pub fn screen_to_world(&self, screen_pos: Vector2<f32>) -> Vector2<f32> {
        Vector2::new(
            (screen_pos.x - screen_width() / 2.0) / self.zoom + self.position.x,
            -(screen_pos.y - screen_height() / 2.0) / self.zoom + self.position.y,
        )
    }
}