/*
Copyright (C) 2022-2025 Amari Calipso

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

use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};
use raylib::math::Vector2;

use crate::{INTERNAL_RESOLUTION, NO_OBSTACLES_CENTER_ZONE, OBSTACLE_GRID_DIV_X, OBSTACLE_GRID_DIV_Y, OBSTACLE_POS_VARIANCE, OBSTACLE_SAFE_ZONE};

#[derive(Debug)]
pub struct ObstacleGrid {
    free: Vec<Vector2>,
    active: HashMap<u16, Vector2>,
    curr_id: u16
}

impl ObstacleGrid {
    fn init_free() -> Vec<Vector2> {
        let resolution_x = INTERNAL_RESOLUTION.x as i32;
        let resolution_y = INTERNAL_RESOLUTION.y as i32;

        let safe_offset_x = OBSTACLE_SAFE_ZONE.x as i32;
        let safe_offset_y = OBSTACLE_SAFE_ZONE.y as i32;

        let no_obst_zone_start_x = (INTERNAL_RESOLUTION.x / 2.0 - NO_OBSTACLES_CENTER_ZONE.x / 2.0) as i32;
        let no_obst_zone_start_y = (INTERNAL_RESOLUTION.y / 2.0 - NO_OBSTACLES_CENTER_ZONE.y / 2.0) as i32;

        let no_obst_zone_size_x = NO_OBSTACLES_CENTER_ZONE.x as i32;
        let no_obst_zone_size_y = NO_OBSTACLES_CENTER_ZONE.y as i32;

        let step_y = resolution_y as usize / OBSTACLE_GRID_DIV_Y as usize;
        let step_x = resolution_x as usize / OBSTACLE_GRID_DIV_X as usize;

        let mut free = Vec::new();

        for y in (safe_offset_y ..= resolution_y - safe_offset_y + step_y as i32).step_by(step_y) {
            for x in (safe_offset_x ..= resolution_x - safe_offset_x + step_x as i32).step_by(step_x) {
                if no_obst_zone_start_x < x && x < no_obst_zone_start_x + no_obst_zone_size_x &&
                   no_obst_zone_start_y < y && y < no_obst_zone_start_y + no_obst_zone_size_y 
                {
                    continue;
                }

                free.push(Vector2 { x: x as f32, y: y as f32 });
            }
        }

        free
    }

    pub fn new() -> Self {
        let free = Self::init_free();

        Self {
            active: HashMap::with_capacity(free.len()),
            curr_id: 0,
            free,
        }
    }

    pub fn reset(&mut self) {
        self.free = Self::init_free();
        self.active.clear();
        self.curr_id = 0;
    }

    pub fn alloc(&mut self, mut player_pos: Vector2, rng: &mut ThreadRng) -> Option<(u16, Vector2)> {
        if self.free.is_empty() {
            return None;
        }

        player_pos.x = (player_pos.x / OBSTACLE_GRID_DIV_X).floor() * OBSTACLE_GRID_DIV_X;
        player_pos.y = (player_pos.y / OBSTACLE_GRID_DIV_Y).floor() * OBSTACLE_GRID_DIV_Y;

        let tmp_pos = self.free.swap_remove(rng.random_range(0..self.free.len()));
        let mut pos = {
            // don't spawn obstacles in the same cell as the player's
            if tmp_pos == player_pos {
                let (_, pos) = self.alloc(player_pos, rng)?;
                self.free.push(tmp_pos);
                pos
            } else {
                tmp_pos
            }
        };

        let id = self.curr_id;
        self.curr_id = self.curr_id.wrapping_add(1);
        self.active.insert(id, pos);

        pos.x += rng.random_range(-OBSTACLE_POS_VARIANCE..=OBSTACLE_POS_VARIANCE);
        pos.y += rng.random_range(-OBSTACLE_POS_VARIANCE..=OBSTACLE_POS_VARIANCE);
        
        Some((id, pos))
    }

    pub fn free(&mut self, id: u16) -> Result<(), ()> {
        if let Some(pos) = self.active.remove(&id) {
            self.free.push(pos);
            Ok(())
        } else {
            Err(())
        }
    }
}