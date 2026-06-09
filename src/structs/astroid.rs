use macroquad::miniquad::window::screen_size;
use macroquad::rand::gen_range;
use macroquad::{color::Color, shapes::draw_poly_lines};

use crate::traits::drawable::Drawable;
use crate::traits::updatable::Updatable;

pub struct Astroid {
    x: f32,
    y: f32,
    size: f32,
    sides: u8,
    rotation: f32,
    rotation_speed: f32,
    direction_x: f32,
    direction_y: f32,
}

impl Astroid {
    pub fn new_at_random_position(x: f32, y: f32) -> Self {
        Self {
            x: gen_range(x - 1000., x + 1000.),
            y: gen_range(y - 1000., y + 1000.),
            size: gen_range(10., 100.),
            sides: gen_range(4, 10),
            rotation: gen_range(0., 360.),
            rotation_speed: gen_range(-30.0, 30.0),
            direction_x: gen_range(-1., 1.),
            direction_y: gen_range(-1., 1.),
        }
    }
}

impl Drawable for Astroid {
    fn draw(&self) {
        let radius = self.size;
        draw_poly_lines(self.x, self.y, self.sides, radius, self.rotation, 2., Color::from_rgba(255, 255, 255, 255));
    }
}

impl Updatable for Astroid {
    fn update(&mut self, context: &crate::structs::context::Context) {
        self.rotation += self.rotation_speed * context.delta_time;
        self.x += self.direction_x * context.delta_time * 100.0;
        self.y += self.direction_y * context.delta_time * 100.0;
    }
}
