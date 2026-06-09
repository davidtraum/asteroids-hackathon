use macroquad::input::mouse_position;
use macroquad::math::{vec2, Vec2};
use macroquad::{color::Color, shapes::draw_triangle};

use crate::traits::drawable::Drawable;
use crate::traits::updatable::Updatable;
use crate::utils::angles::{lengthdir, point_direction, to_rad};

pub struct Spaceship {
    pub pos: Vec2,
    pub rotation: f32,
}

impl Spaceship {
    pub fn new(pos: Vec2) -> Self {
        Spaceship {
            pos: pos,
            rotation: 0.,
        }
    }
}

impl Drawable for Spaceship {
    fn draw(&self) {
        draw_triangle(
            self.pos,
            self.pos + lengthdir(60., self.rotation + to_rad(-20.)),
            self.pos + lengthdir(60., self.rotation + to_rad(-110.)),
            // 2.,
            Color::from_rgba(255, 255, 255, 255),
        );
    }
}

impl Updatable for Spaceship {
    fn update(&mut self, _context: &crate::structs::context::Context) {
        let mp = mouse_position();
        let mp = vec2(mp.0, mp.1);
        self.rotation = point_direction(self.pos, mp);
    }
}
