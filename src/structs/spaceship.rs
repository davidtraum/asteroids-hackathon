use macroquad::{color::Color, shapes::draw_rectangle};

use crate::traits::drawable::Drawable;
use crate::traits::updatable::Updatable;

pub struct Spaceship {
    x: f32,
    y: f32,
}

impl Spaceship {
    pub fn new() -> Self {
        Spaceship { x: 0., y: 0. }
    }
}

impl Drawable for Spaceship {
    fn draw(&self) {
        draw_rectangle(
            self.x,
            self.y,
            self.x + 10.,
            self.y + 10.,
            Color::from_rgba(255, 255, 255, 255),
        );
    }
}

impl Updatable for Spaceship {
    fn update(&mut self, _context: &crate::structs::context::Context) {}
}
