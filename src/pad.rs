use rand::{rngs::ThreadRng, Rng};
use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::{EFFECTIVE_PAD_FRMT, FG, INTERNAL_RESOLUTION, OBSTACLE_SAFE_ZONE, PAD_SIZE, PAD_WALL_DISTANCE, PLAYER_SIZE};

#[derive(Debug)]
pub struct Pad {
    pub pos: Vector2,
    size: Vector2,

    is_left: bool,

    cnt: f32,
    step: f32
}

impl Pad {
    pub fn default(is_left: bool) -> Self {
        let pos = {
            if is_left {
                Vector2 {
                    x: PAD_WALL_DISTANCE,
                    y: INTERNAL_RESOLUTION.y / 2.0 - PAD_SIZE.y / 2.0
                }
            } else {
                Vector2 {
                    x: INTERNAL_RESOLUTION.x - PAD_WALL_DISTANCE - PAD_SIZE.x,
                    y: INTERNAL_RESOLUTION.y / 2.0 - PAD_SIZE.y / 2.0
                }
            }
        };

        Self {
            pos,
            is_left,
            size: PAD_SIZE,
            cnt: 0.0,
            step: 0.0,
        }
    }

    #[allow(unused)]
    pub fn new(is_left: bool, pos: Vector2, size: Vector2) -> Self {
        Self {
            pos,
            is_left,
            size,
            cnt: 0.0,
            step: 0.0,
        }
    }

    fn move_to(&mut self, pos: f32) {
        if self.size == PAD_SIZE {
            self.cnt = 0.0;
            self.step = (pos - self.pos.y) / EFFECTIVE_PAD_FRMT;
        }
    }

    pub fn reset(&mut self) {
        self.pos.y = INTERNAL_RESOLUTION.y / 2.0 - self.size.y / 2.0;
        self.step = 0.0;
    }

    pub fn update(&mut self, delta_time: f32, draw: &mut impl RaylibDraw) {
        self.pos.y += self.step * delta_time;

        self.cnt += delta_time;
        if self.cnt >= EFFECTIVE_PAD_FRMT {
            self.step = 0.0;
        }

        draw.draw_rectangle(
            self.pos.x as i32, self.pos.y as i32, 
            self.size.x as i32, self.size.y as i32, 
            FG
        );
    }

    pub fn collides(&mut self, player_pos: Vector2, tolerance: f32, rng: &mut ThreadRng) -> bool {
        let r = self.pos.y .. self.pos.y + self.size.y;

        if self.is_left {
            let tmp = self.pos.x + self.size.x;

            if (tmp - tolerance .. tmp).contains(&player_pos.x) && 
               (r.contains(&player_pos.y) || r.contains(&(player_pos.y + PLAYER_SIZE))) 
            {
                self.move_to(rng.random_range(OBSTACLE_SAFE_ZONE.y .. INTERNAL_RESOLUTION.y - OBSTACLE_SAFE_ZONE.y - self.size.y));
                return true;
            }
        } else {
            let player_pos_x = player_pos.x + PLAYER_SIZE;

            if (self.pos.x .. self.pos.x + tolerance).contains(&player_pos_x) &&
               (r.contains(&player_pos.y) || r.contains(&(player_pos.y + PLAYER_SIZE)))
            {
                self.move_to(rng.random_range(OBSTACLE_SAFE_ZONE.y .. INTERNAL_RESOLUTION.y - OBSTACLE_SAFE_ZONE.y - self.size.y));
                return true;
            }
        }

        false
    }
}