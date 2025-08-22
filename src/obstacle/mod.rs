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

use enum_dispatch::enum_dispatch;
use rand::rngs::ThreadRng;
use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::{obstacle::{explosion::ObstacleExplosion, rock::Rock, rocket::Rocket}, FrameInfo};

pub mod rock;
pub mod rocket;
pub mod explosion;

#[enum_dispatch(AnyObstacle)]
pub trait Obstacle {
    fn can_collide(&self) -> bool {
        true
    }

    fn pos(&self) -> Vector2;
    fn size(&self) -> Vector2;

    fn is_alive(&self) -> bool;
    fn update(&mut self, frame_info: FrameInfo, rng: &mut ThreadRng, draw: &mut impl RaylibDraw);
    
    fn kill(&mut self);
    fn collides_object(&mut self, pos: Vector2, size: Vector2) -> bool;
    fn collides_other(&mut self, other: &AnyObstacle) -> bool;
}

#[enum_dispatch]
#[derive(Debug)]
pub enum AnyObstacle {
    Rock,
    Rocket,
    ObstacleExplosion
}
