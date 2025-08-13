use enum_dispatch::enum_dispatch;
use rand::rngs::ThreadRng;
use raylib::{math::Vector2, prelude::RaylibDraw};

use crate::obstacle::{explosion::ObstacleExplosion, rock::Rock, rocket::Rocket};

pub mod rock;
pub mod rocket;
pub mod explosion;

#[enum_dispatch(AnyObstacle)]
pub trait Obstacle {
    fn pos(&self) -> Vector2;
    fn size(&self) -> Vector2;
    fn can_collide(&self) -> bool;

    fn is_alive(&self) -> bool;
    fn update(&mut self, delta_time: f32, rng: &mut ThreadRng, draw: &mut impl RaylibDraw);
    
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
