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

use raylib::math::Vector2;

pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2 { x, y }
}

#[macro_export]
macro_rules! get_expect {
    ($obj: expr) => {
        $obj.get().expect(concat!("Could not get Once expression: ", stringify!($obj)))
    };
}

#[macro_export]
macro_rules! get_expect_mut {
    ($obj: expr) => {
        $obj.get_mut().expect(concat!("Could not get Once expression: ", stringify!($obj)))
    };
}

pub fn square_collides(pos: Vector2, size: Vector2, other_pos: Vector2, other_size: Vector2) -> bool {
    let yr = pos.y .. pos.y + size.y;
    let xr = pos.x .. pos.x + size.x;
    (xr.contains(&other_pos.x) || xr.contains(&(other_pos.x + other_size.x))) &&
    (yr.contains(&other_pos.y) || yr.contains(&(other_pos.y + other_size.y)))
}