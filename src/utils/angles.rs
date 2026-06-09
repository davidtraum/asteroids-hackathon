use macroquad::math::{vec2, Vec2};

pub fn point_direction(pos1: Vec2, pos2: Vec2) -> f32 {
    let diff = pos2 - pos1;
    (diff.x / diff.y).atan()
}

pub fn lengthdir(distance: f32, direction: f32) -> Vec2 {
    vec2(direction.cos() * distance, direction.sin() * distance)
}

pub fn to_rad(degrees: f32) -> f32 {
    degrees / 180. * std::f32::consts::PI
}
