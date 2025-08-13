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

    fn update(&mut self, delta_time:f32, _rng: &mut ThreadRng, draw: &mut impl RaylibDraw) {
        self.0.update(delta_time);
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