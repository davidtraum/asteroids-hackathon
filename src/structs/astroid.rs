use macroquad::shapes::draw_circle;
use macroquad::{color::Color, shapes::draw_rectangle};

use crate::traits::drawable::Drawable;
use crate::traits::updatable::Updatable;

pub struct Astroid {
    x: f32,
    y: f32,
    size: f32,
}

impl Astroid {
    pub fn new_at_random_position() -> Self {
        Self {
            x: 0.,
            y: 0.,
            size: 5.,
        }
    }
}

impl Drawable for Astroid {
    fn draw(&self) {
        let radius = self.size;
        draw_circle(self.x, self.y, radius, Color::from_rgba(255, 255, 255, 255));
    }
}

impl Updatable for Astroid {
    fn update(&mut self) {}
}
