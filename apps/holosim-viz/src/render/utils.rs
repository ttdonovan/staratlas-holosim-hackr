use macroquad::prelude::*;
use nalgebra::Vector2;

pub fn transform_point(matrix: &Mat4, point: &Vector2<f32>) -> Vector2<f32> {
    let transformed = *matrix * vec4(point.x, -point.y, 0.0, 1.0);
    Vector2::new(transformed.x, transformed.y)
}