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

use raylib::texture::Image;

const CUTOFF: u8 = 3;

const THRESHOLDS: [[u8; 8]; 8] = {
    let mut t = [
        [ 0, 32,  8, 40,  2, 34, 10, 42],
        [48, 16, 56, 24, 50, 18, 58, 26],
        [12, 44,  4, 36, 14, 46,  6, 38],
        [60, 28, 52, 20, 62, 30, 54, 22],
        [ 3, 35, 11, 43,  1, 33,  9, 41],
        [51, 19, 59, 27, 49, 17, 57, 25],
        [15, 47,  7, 39, 13, 45,  5, 37],
        [63, 31, 55, 23, 61, 29, 53, 21]
    ];

    let mut y = 0;
    while y < t.len() {
        let mut x = 0;
        while x < t[y].len() {
            t[y][x] += CUTOFF;
            x += 1;
        }

        y += 1;
    }

    t
};

pub fn apply(image: &mut Image) {
    let width = image.width as usize;
    let height = image.height as usize;
    let size = width * height * 4;
    let data = unsafe { std::slice::from_raw_parts_mut(image.data as *mut u8, size) };

    for y in 0 .. image.height as usize {
        for x in 0 .. width {
            let t = THRESHOLDS[y % THRESHOLDS.len()][x % THRESHOLDS[0].len()];

            let i = 4 * (y * width + x);
            data[i + 0] = (data[i + 0] > t) as u8 * 255;
            data[i + 1] = (data[i + 1] > t) as u8 * 255;
            data[i + 2] = (data[i + 2] > t) as u8 * 255;
            // don't edit alpha
        }
    }
}