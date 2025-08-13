use rand::{rngs::ThreadRng, Rng};
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

use crate::{obstacle::{AnyObstacle, Obstacle}, utils::{square_collides, vec2}, FG, MAX_OBSTACLE_LIFE, MAX_OBSTACLE_SIZE, MIN_OBSTACLE_LIFE, MIN_OBSTACLE_SIZE, OBSTACLE_DELTA_ALPHA, OBSTACLE_START_ALPHA};

#[derive(Debug)]
pub struct Rock {
    pub id: u16,

    pos: Vector2,
    size: Vector2,

    lifespan: f32,
    step: i8,
    alpha: u8
}

impl Rock {
    pub fn new(rng: &mut ThreadRng, id: u16, pos: Vector2) -> Self {
        let tmp = rng.random_range(MIN_OBSTACLE_SIZE..=MAX_OBSTACLE_SIZE);
        let size = vec2(tmp, tmp);
        let pos = pos - size;

        Rock {
            pos, size, id,
            lifespan: rng.random_range(MIN_OBSTACLE_LIFE..=MAX_OBSTACLE_LIFE),
            step: OBSTACLE_DELTA_ALPHA,
            alpha: OBSTACLE_START_ALPHA
        }
    }
}

impl Obstacle for Rock {
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
        self.alpha = 0;
    }

    fn is_alive(&self) -> bool {
        self.alpha > 0
    }

    fn update(&mut self, delta_time: f32, _rng: &mut ThreadRng, draw: &mut impl RaylibDraw) {
        if self.lifespan <= 0.0 {
            self.step = (-(OBSTACLE_DELTA_ALPHA as f32) * delta_time) as i8;
        }

        self.lifespan -= delta_time;
        self.alpha = self.alpha.saturating_add_signed(self.step);

        draw.draw_rectangle(
            self.pos.x as i32, self.pos.y as i32, 
            self.size.x as i32, self.size.y as i32, 
            Color { r: FG.r, g: FG.g, b: FG.b, a: self.alpha }
        );
    }

    fn collides_object(&mut self, pos: Vector2, size: Vector2) -> bool {
        square_collides(self.pos, self.size, pos, size)
    }

    fn collides_other(&mut self, other: &AnyObstacle) -> bool {
        square_collides(self.pos, self.size, other.pos(), other.size())
    }
}