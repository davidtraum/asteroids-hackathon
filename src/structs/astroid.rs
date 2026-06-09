use macroquad::math::{vec2, Vec2};
use macroquad::rand::gen_range;
use macroquad::{color::Color, shapes::draw_poly_lines};

use crate::traits::drawable::Drawable;
use crate::traits::updatable::Updatable;

pub struct Astroid {
    pos: Vec2,
    size: f32,
    sides: u8,
    rotation: f32,
    rotation_speed: f32,
    direction: Vec2,
}

impl Astroid {
    pub fn new_at_random_position(pos: Vec2) -> Self {
        let spawn_above_screen = gen_range(0, 1) > 0;
        let spawn_left_of_screen = gen_range(0, 1) > 0;
        let max_outside_bounds = 1000.;
        let min_outside_bounds = 100.;
        let dynamic_param = vec2(
            gen_range(min_outside_bounds, max_outside_bounds),
            gen_range(min_outside_bounds, max_outside_bounds),
        );

        let flip_value = vec2(
            if spawn_left_of_screen { -1. } else { 1. },
            if spawn_above_screen { -1. } else { 1. },
        );
        let my_pos = pos + dynamic_param * flip_value;
        Self {
            pos: my_pos,
            size: gen_range(10., 100.),
            sides: gen_range(5, 10),
            rotation: gen_range(0., 360.),
            rotation_speed: gen_range(-30.0, 30.0),
            direction: vec2(gen_range(1., 5.), gen_range(1., 5.)) * flip_value * -1.,
        }
    }
}

impl Drawable for Astroid {
    fn draw(&self) {
        let radius = self.size;
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            self.sides,
            radius,
            self.rotation,
            2.,
            Color::from_rgba(255, 255, 255, 255),
        );
    }
}

impl Updatable for Astroid {
    fn update(&mut self, context: &crate::structs::context::Context) {
        self.rotation += self.rotation_speed * context.delta_time;
        self.pos += self.direction * context.delta_time * 100.0;
    }
}
