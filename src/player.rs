/*
Copyright (C) 2025 Amari Calipso

This file is part of !pong.

!pong is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

!pong is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with !pong.  If not, see <http://www.gnu.org/licenses/>.
 */

use rand::{rngs::ThreadRng, Rng};
use raylib::{color::Color, math::{Rectangle, Vector2}, prelude::RaylibDraw};

use crate::{explosion::Explosion, pad::Pad, FrameInfo, DEATH_MAX_INIT_PARTICLE_VELOCITY, FG, GRAVITY, HOVER_RAINBOW_DELTA, HOVER_RAINBOW_DISTANCE, HOVER_SPACE, INTERNAL_RESOLUTION, JUMP_VELOCITY, PLAYER_SIZE, PLAYER_VELOCITY, RAINBOW_DELTA, SPRINT_VELOCITY};

#[derive(Debug)]
pub struct Player {
    hover_angle: f32,
    
    pub count: u64,
    pub playing: bool,
    pub sprinting: bool,
    pub dead: bool,
    rainbow: bool,
    rainbow_cnt: f32,

    pub pos: Vector2,
    velocity: Vector2,

    pub explosion: Explosion
}

impl Player {
    fn base_velocity(rng: &mut ThreadRng) -> Vector2 {
        if rng.random_bool(0.5) {
            PLAYER_VELOCITY
        } else {
            -PLAYER_VELOCITY
        }
    }

    pub fn new() -> Self {
        let pos = Vector2 { 
            x: INTERNAL_RESOLUTION.x / 2.0, 
            y: INTERNAL_RESOLUTION.y / 2.0 
        };

        Self {
            hover_angle: 0.0,
            count: 0,
            playing: false,
            sprinting: false,
            dead: false,
            rainbow: false,
            rainbow_cnt: 0.0,
            pos,
            velocity: Vector2::zero(),
            explosion: Explosion::new(pos),
        }  
    }

    pub fn init(&mut self, rng: &mut ThreadRng) {
        self.velocity = Self::base_velocity(rng);
    }

    fn reset_pos(&mut self) {
        self.pos = Vector2 { 
            x: INTERNAL_RESOLUTION.x / 2.0, 
            y: INTERNAL_RESOLUTION.y / 2.0 
        };
    }

    pub fn reset(&mut self, rng: &mut ThreadRng) {
        self.hover_angle = 0.0;
        self.count = 0;
        self.playing = false;
        self.dead = false;
        self.sprinting = false;
        self.rainbow = false;
        self.rainbow_cnt = 0.0;

        self.velocity = Self::base_velocity(rng);
        self.reset_pos();
    }

    pub fn sprint_on(&mut self) {
        self.sprinting = true;
        self.velocity.y = 0.0;
    }

    pub fn sprint_off(&mut self) {
        self.sprinting = false;
    }

    pub fn start(&mut self, rng: &mut ThreadRng) {
        self.playing = true;
        self.reset_pos();
        self.velocity = Self::base_velocity(rng);
    }

    pub fn invert(&mut self) {
        self.count += 1;
        self.velocity.x = -self.velocity.x;
    }

    pub fn jump(&mut self, rng: &mut ThreadRng) {
        if !self.playing {
            self.start(rng);
        }

        if !self.sprinting {
            self.velocity.y = -JUMP_VELOCITY;
        }
    }

    pub fn is_dead(&mut self, left_pad: &Pad, right_pad: &Pad, tolerance: f32, rng: &mut ThreadRng) -> bool {
        if self.dead {
            self.explosion.explode_with_pos(
                self.pos, 
                DEATH_MAX_INIT_PARTICLE_VELOCITY, 
                false, rng
            );

            self.reset(rng);
            return true;
        }

        if self.pos.x <= 0.0 {
            // avoids collision problems with low framerate
            if !left_pad.collides(self.pos, tolerance) {
                self.explosion.explode_with_pos(
                    Vector2 { x: 0.0, y: self.pos.y }, 
                    DEATH_MAX_INIT_PARTICLE_VELOCITY, 
                    false, rng
                );
            }
        } else if self.pos.x + PLAYER_SIZE >= INTERNAL_RESOLUTION.x {
            self.explosion.explode_with_pos(
                Vector2 { 
                    x: INTERNAL_RESOLUTION.x - 1.0, 
                    y: self.pos.y 
                }, 
                DEATH_MAX_INIT_PARTICLE_VELOCITY,
                false, rng
            );
        } else if self.pos.y <= 0.0 {
            self.explosion.explode_with_pos(
                Vector2 { x: self.pos.x, y: 0.0 }, 
                DEATH_MAX_INIT_PARTICLE_VELOCITY, 
                false, rng
            );
        } else if self.pos.y + PLAYER_SIZE >= INTERNAL_RESOLUTION.y {
            // avoids collision problems with low framerate
            if !right_pad.collides(self.pos, tolerance) {
                self.explosion.explode_with_pos(
                    Vector2 { 
                        x: self.pos.x, 
                        y: INTERNAL_RESOLUTION.y - 1.0 
                    }, 
                    DEATH_MAX_INIT_PARTICLE_VELOCITY, 
                    false, rng
                );
            }
        } else {
            return false;
        }

        self.reset(rng);
        true
    }

    fn dir(&self, x: f32) -> f32 {
        if self.velocity.x > 0.0 {
            x
        } else {
            -x
        }
    }

    pub fn update(&mut self, frame_info: FrameInfo, rng: &mut ThreadRng, draw: &mut impl RaylibDraw) {
        if self.explosion.is_alive() {
            self.explosion.update(frame_info);
            self.explosion.show(draw);
        } else {
            if self.playing {
                if self.sprinting {
                    self.pos.x += self.dir(SPRINT_VELOCITY) * frame_info.delta_time;
                } else {
                    self.velocity += GRAVITY * frame_info.delta_time;
                    self.pos += self.velocity * frame_info.delta_time;
                }

                if self.sprinting {
                    draw.draw_rectangle(
                        self.pos.x as i32, self.pos.y as i32, 
                        PLAYER_SIZE as i32, PLAYER_SIZE as i32, 
                        Color::color_from_hsv(self.rainbow_cnt * 360.0, 1.0, 1.0)
                    );

                    self.rainbow_cnt += RAINBOW_DELTA * frame_info.delta_time;
                    if self.rainbow_cnt > 1.0 {
                        self.rainbow_cnt = 0.0;
                    }
                } else {
                    draw.draw_rectangle(
                        self.pos.x as i32, self.pos.y as i32, 
                        PLAYER_SIZE as i32, PLAYER_SIZE as i32, 
                        FG
                    );
                }
            } else {
                // this shouldn't happen, but it does and i have no idea why
                if self.velocity.x as i32 == 0 {
                    if self.pos.x < INTERNAL_RESOLUTION.x / 2.0 {
                        self.velocity = PLAYER_VELOCITY;
                    } else {
                        self.velocity = -PLAYER_VELOCITY;
                    }
                }

                if self.pos.y > INTERNAL_RESOLUTION.y / 2.0 + HOVER_SPACE {
                    self.velocity.y = -JUMP_VELOCITY;
                }

                if self.pos.x + PLAYER_SIZE + frame_info.tolerance >= INTERNAL_RESOLUTION.x || self.pos.x <= frame_info.tolerance {
                    self.velocity.x = -self.velocity.x;
                }

                self.velocity += GRAVITY * frame_info.delta_time;
                self.pos += self.velocity * frame_info.delta_time;

                let mut color0 = Color::color_from_hsv(self.rainbow_cnt * 360.0, 1.0, 1.0);
                let mut color1 = Color::color_from_hsv((self.rainbow_cnt - HOVER_RAINBOW_DISTANCE) * 360.0, 1.0, 1.0);

                if rng.random_bool(0.5) {
                    std::mem::swap(&mut color0, &mut color1);
                }

                let color2;
                let color3;
                if rng.random_bool(0.5) {
                    color2 = color0;
                    color3 = color1;
                } else {
                    color2 = color1;
                    color3 = color0;
                }

                draw.draw_rectangle_gradient_ex(
                    Rectangle::new(
                        self.pos.x, self.pos.y, 
                        PLAYER_SIZE, PLAYER_SIZE, 
                    ),
                    color0, color1, color2, color3,
                );

                self.rainbow_cnt += HOVER_RAINBOW_DELTA * frame_info.delta_time;
                if self.rainbow_cnt > 1.0 {
                    self.rainbow_cnt = 0.0;
                }
            }
        }
    }
}