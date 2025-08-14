use raylib::{color::Color, math::{Rectangle, Vector2}, prelude::RaylibDraw};

use crate::{utils::{square_collides, vec2}, BOMB_ANGLE_INCREMENT, BOMB_LIFE, BOMB_SIZE, RAINBOW_DELTA};

#[derive(Debug)]
pub struct Bomb {
    pub pos: Vector2,
    velocity: Vector2,

    angle: f32,
    lifetime: f32,

    color_cnt: f32,

    pub to_destroy: Vec<Rectangle>,
    pub give_points: u64,
}

impl Bomb {
    pub fn new(pos: Vector2) -> Self {
        let pos = pos - BOMB_SIZE / 2.0;

        Self {
            pos,
            velocity: Vector2::zero(),
            angle: 0.0,
            color_cnt: 0.0,
            lifetime: BOMB_LIFE,
            to_destroy: Vec::new(),
            give_points: 0
        }
    }

    pub fn collides(&self, pos: Vector2, size: Vector2) -> bool {
        square_collides(self.pos, vec2(BOMB_SIZE, BOMB_SIZE), pos, size)
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn update(&mut self, delta_time: f32, draw: &mut impl RaylibDraw) {
        if self.to_destroy.len() > 0 {
            let to_destroy = *self.to_destroy.last().unwrap();
            let to_destroy_pos = Vector2 { x: to_destroy.x, y: to_destroy.y };
            let to_destroy_size = Vector2 { x: to_destroy.width, y: to_destroy.height };

            if self.collides(to_destroy_pos, to_destroy_size) {
                self.to_destroy.pop();

                // destroy bomb immediately when it gets to position if it's for points
                if self.give_points > 0 {
                    self.lifetime = 0.0;
                }
            } else {
                self.velocity -= (self.pos - to_destroy_pos) * 0.0025;
                self.pos += self.velocity;
                self.velocity *= 0.95;
            }
        } else {
            self.lifetime -= delta_time;
        }

        let color = Color::color_from_hsv(self.color_cnt * 360.0, 1.0, 1.0);

        const HALF_SIZE: f32 = BOMB_SIZE / 2.0;
        let side_size = self.angle.sin() * HALF_SIZE;
        let center_x = self.pos.x + HALF_SIZE;

        let v0 = Vector2 {
            x: center_x,
            y: self.pos.y
        };

        let v1 = Vector2 {
            x: center_x,
            y: self.pos.y + BOMB_SIZE
        };

        let vl = Vector2 {
            x: center_x - side_size,
            y: self.pos.y + HALF_SIZE
        };

        let vr = Vector2 {
            x: center_x + side_size,
            y: self.pos.y + HALF_SIZE
        };

        if side_size < 0.0 {
            draw.draw_triangle(v0, vr, vl, color);
            draw.draw_triangle(v1, vl, vr, color);
        } else {
            draw.draw_triangle(v0, vl, vr, color);
            draw.draw_triangle(v1, vr, vl, color);
        }
        
        self.angle += BOMB_ANGLE_INCREMENT * delta_time;
        self.color_cnt += RAINBOW_DELTA * delta_time;
        if self.color_cnt > 1.0 {
            self.color_cnt = 0.0;
        }
    }
}