use macroquad::rand::gen_range;
use macroquad::{color::Color, shapes::draw_poly_lines};
use macroquad::window;

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
        let spawn_above_screen = gen_range(0,1) > 0;
        let spawn_left_of_screen = gen_range(0,1) > 0;
        let max_outside_bounds = 1000.;
        let min_outside_bounds = 100.;
        let offset_x = if (spawn_left_of_screen) { x - window::screen_width() * 0.5 - gen_range(min_outside_bounds, max_outside_bounds ) } else { x + window::screen_height() * 0.5 + gen_range(min_outside_bounds, max_outside_bounds)};
        let offset_y = if (spawn_above_screen) { y - window::screen_height() * 0.5 - gen_range(min_outside_bounds, max_outside_bounds) } else { y + window::screen_height() * 0.5 + gen_range(min_outside_bounds, max_outside_bounds) };
        Self {
            x: offset_x,
            y: offset_y,
            size: gen_range(10., 100.),
            sides: gen_range(5, 10),
            rotation: gen_range(0., 360.),
            rotation_speed: gen_range(-30.0, 30.0),
            direction_x: gen_range(if offset_x < x {0.} else {1.}, if offset_x < x {1.} else {0.}),
            direction_y: gen_range(if offset_y < y {0.} else {1.}, if offset_y < y {1.} else {0.}),
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
