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

use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::{obstacle::{AnyObstacle, Obstacle}, utils::{square_collides, vec2}, FG, INTERNAL_RESOLUTION, MAX_OBSTACLE_SIZE, MAX_ROCKET_SPEED, MIN_OBSTACLE_SIZE, MIN_ROCKET_SPEED, ROCKET_SHAKE};

#[derive(Debug)]
pub enum Base {
    Left, Right, Top, Bottom
}

impl Base {
    pub fn random(rng: &mut ThreadRng) -> Self {
        [Base::Left, Base::Right, Base::Top, Base::Bottom].into_iter().choose(rng).unwrap()
    }
}

#[derive(Debug)]
pub struct Rocket {
    pub id: u16,

    pub pos: Vector2,
    size: Vector2,
    velocity: f32,
    base: Base,

    dead: bool
}

impl Rocket {
    pub fn new(rng: &mut ThreadRng, id: u16, pos: Vector2, base: Base) -> Self {
        let tmp = rng.random_range(MIN_OBSTACLE_SIZE..=MAX_OBSTACLE_SIZE);

        Self {
            id, pos, base,
            size: vec2(tmp, tmp),
            velocity: rng.random_range(MIN_ROCKET_SPEED..=MAX_ROCKET_SPEED),
            dead: false
        }
    }
}

impl Obstacle for Rocket {
    fn can_collide(&self) -> bool {
        true
    }
    
    fn pos(&self) -> Vector2 {
        self.pos
    }

    fn size(&self) -> Vector2 {
        self.size
    }

    fn kill(&mut self) {
        self.dead = true;
    }

    fn is_alive(&self) -> bool {
        if self.dead {
            return false;
        }

        match self.base {
            Base::Left => self.pos.x < INTERNAL_RESOLUTION.x,
            Base::Right => self.pos.x > 0.0,
            Base::Top => self.pos.y < INTERNAL_RESOLUTION.y,
            Base::Bottom => self.pos.y > 0.0,
        }
    }

    fn update(&mut self, delta_time: f32, rng: &mut ThreadRng, draw: &mut impl RaylibDraw) {
        match self.base {
            Base::Left => {
                self.pos.x += self.velocity * delta_time;

                let mut draw_pos = self.pos;
                draw_pos.y += rng.random_range(-ROCKET_SHAKE..=ROCKET_SHAKE);
                
                draw.draw_triangle(
                    draw_pos, 
                    Vector2 { 
                        x: draw_pos.x, 
                        y: draw_pos.y + self.size.y 
                    }, 
                    Vector2 { 
                        x: draw_pos.x + self.size.x, 
                        y: draw_pos.y + self.size.y / 2.0 
                    }, 
                    FG
                );
            }
            Base::Right => {
                self.pos.x -= self.velocity * delta_time;

                let mut draw_pos = self.pos;
                draw_pos.y += rng.random_range(-ROCKET_SHAKE..=ROCKET_SHAKE);

                draw.draw_triangle(
                    Vector2 { 
                        x: draw_pos.x + self.size.x, 
                        y: draw_pos.y
                    },
                    Vector2 { 
                        x: draw_pos.x, 
                        y: draw_pos.y + self.size.y / 2.0
                    }, 
                    draw_pos + self.size,
                    FG
                );
            }
            Base::Top => {
                self.pos.y += self.velocity * delta_time;

                let mut draw_pos = self.pos;
                draw_pos.x += rng.random_range(-ROCKET_SHAKE..=ROCKET_SHAKE);

                draw.draw_triangle(
                    Vector2 { 
                        x: draw_pos.x + self.size.x, 
                        y: draw_pos.y
                    },
                    draw_pos,
                    Vector2 { 
                        x: draw_pos.x + self.size.x / 2.0, 
                        y: draw_pos.y + self.size.y
                    }, 
                    FG
                );
            }
            Base::Bottom => {
                self.pos.y -= self.velocity * delta_time;

                let mut draw_pos = self.pos;
                draw_pos.x += rng.random_range(-ROCKET_SHAKE..=ROCKET_SHAKE);

                draw.draw_triangle(
                    Vector2 { 
                        x: draw_pos.x + self.size.x / 2.0, 
                        y: draw_pos.y
                    }, 
                    Vector2 { 
                        x: draw_pos.x, 
                        y: draw_pos.y + self.size.y
                    },
                    draw_pos + self.size,
                    FG
                );
            }
        }
    }

    fn collides_object(&mut self, pos: Vector2, size: Vector2) -> bool {
        square_collides(self.pos, self.size, pos, size)
    }

    fn collides_other(&mut self, other: &AnyObstacle) -> bool {
        square_collides(self.pos, self.size, other.pos(), other.size())
    }
}