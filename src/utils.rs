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