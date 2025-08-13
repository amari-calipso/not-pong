use lazy_static::lazy_static;
use raylib::texture::Image;

const CUTOFF: u8 = 3;

lazy_static! {
    static ref THRESHOLDS: [[u8; 8]; 8] = {
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

        for y in t.iter_mut() {
            for item in y.iter_mut() {
                *item += CUTOFF;
            }
        }

        t
    };
}

pub fn apply(image: &mut Image) {
    let width = image.width as usize;
    let height = image.height as usize;
    let size = width * height * 4;
    let data = unsafe { std::slice::from_raw_parts_mut(image.data as *mut u8, size) };

    for y in 0 .. image.height as usize {
        for x in 0 .. width {
            let t = THRESHOLDS[y % THRESHOLDS.len()][x % THRESHOLDS.len()];

            let i = 4 * (y * width + x);
            data[i + 0] = (data[i + 0] > t) as u8 * 255;
            data[i + 1] = (data[i + 1] > t) as u8 * 255;
            data[i + 2] = (data[i + 2] > t) as u8 * 255;
            // don't edit alpha
        }
    }
}