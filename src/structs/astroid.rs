use macroquad::math::{vec2, Vec2};
use macroquad::rand::gen_range;
use macroquad::{color::Color, shapes::draw_poly_lines};

use crate::traits::drawable::Drawable;
use crate::traits::updatable::Updatable;
use crate::utils::angles::to_rad;

pub struct Astroid {
    pub pos: Vec2,
    pub size: f32,
    pub id: i32,
    sides: u8,
    rotation: f32,
    rotation_speed: f32,
    direction: Vec2,
    velocity: f32,
    pub last_collider_id: i32,
    pub current_collider_id: i32,
}

impl Astroid {
    pub fn new_at_random_position(pos: Vec2, id: u32) -> Self {
        let spawn_above_screen = gen_range(0, 2) == 0;
        let spawn_left_of_screen = gen_range(0, 2) == 0;
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
            velocity: gen_range(1, 1000) as f32 / 1000.0,
            direction: vec2(gen_range(1., 5.), gen_range(1., 5.)) * flip_value * -1.,
            id: id as i32,
            last_collider_id: -1,
            current_collider_id: -1,
        }
    }

    pub fn split(&self, base_id: u32, force: bool, collider_id: i32) -> Option<(Self, Self)>{
        let new_size = self.size * 0.5;

        if (!force && new_size < 20.) || collider_id == self.last_collider_id {
            return None;
        }

        Some((
            Self {
                pos: self.pos,
                size: gen_range(new_size - 10., new_size + 10.),
                sides: gen_range(5, 10),
                rotation: self.rotation,
                rotation_speed: self.rotation_speed,
                direction: vec2(self.direction.x, -self.direction.y),
                velocity: self.velocity,
                last_collider_id: self.id,
                id: base_id as i32,
                current_collider_id: -1,
            },
            Self {
                pos: self.pos,
                size: gen_range(new_size - 10., new_size + 10.),
                sides: gen_range(5, 10),
                rotation: self.rotation,
                rotation_speed: self.rotation_speed,
                direction: vec2(-self.direction.x, self.direction.y),
                velocity: self.velocity,
                id: base_id as i32 + 1,
                last_collider_id: self.id,
                current_collider_id: -1,
            }
        ))
    
    }

    pub fn check_collision(&self, point: Vec2, target_radius: f32) -> bool {
        let distance = (self.pos - point).length();
        return distance < (self.size + target_radius);
    }

    pub fn deflect(&mut self, force: bool, collider_id: i32) {
        if(collider_id == self.last_collider_id && !force) {
            return;
        }
        self.direction = self.direction * vec2(gen_range(to_rad(-10.), 
            to_rad(10.)),
            gen_range(to_rad(-10.), 
            to_rad(10.)));
        self.last_collider_id = collider_id;
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
        self.pos += self.direction * context.delta_time * 10.0 * self.velocity;
    }
}
