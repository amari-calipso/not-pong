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

use rand::rngs::ThreadRng;
use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::{explosion::Explosion, obstacle::{AnyObstacle, Obstacle}};

#[derive(Debug)]
pub struct ObstacleExplosion(pub Explosion);
impl Obstacle for ObstacleExplosion {
    fn can_collide(&self) -> bool {
        false
    }

    fn pos(&self) -> Vector2 {
        self.0.pos
    }

    fn size(&self) -> Vector2 {
        Vector2::zero()
    }

    fn is_alive(&self) -> bool {
        self.0.is_alive()
    }

    fn update(&mut self, _delta_time: f32, in_reference_frame: bool, _rng: &mut ThreadRng, draw: &mut impl RaylibDraw) {
        self.0.update(in_reference_frame);
        self.0.show(draw);
    }

    fn kill(&mut self) {
        unreachable!()
    }

    fn collides_object(&mut self, _pos: Vector2, _size: Vector2) -> bool {
        unreachable!()
    }

    fn collides_other(&mut self, _other: &AnyObstacle) -> bool {
        unreachable!()
    }
}