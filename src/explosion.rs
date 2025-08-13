use rand::{rngs::ThreadRng, Rng};
use raylib::{color::Color, math::Vector2, prelude::RaylibDraw};

use crate::{FG, INTERNAL_RESOLUTION, LIFESPAN_DECREASE, MAX_PARTICLE_QTY, MIN_PARTICLE_QTY, PARTICLE_SIZE, PARTICLE_VELOCITY_MULTIPLIER};

#[derive(Debug, Clone)]
pub struct Particle {
    pos: Vector2,
    velocity: Vector2,
    acceleration: Vector2,

    color: Color,
    lifespan: u8,
    alive: bool
}

impl Particle {
    pub fn new(pos: Vector2, max_init_velocity: f32, color: Color, rng: &mut ThreadRng) -> Self {
        let mut velocity = Vector2 {
            x: rng.random_range(-1.0..1.0),
            y: rng.random_range(-1.0..1.0),
        };

        velocity *= rng.random_range(0.1..max_init_velocity);

        Self { 
            pos, 
            velocity, 
            color,
            acceleration: Vector2 { x: 0.0, y: 0.0 }, 
            lifespan: u8::MAX, 
            alive: true
        }
    }

    #[allow(unused)]
    pub fn apply_force(&mut self, f: Vector2) {
        self.acceleration += f;
    }

    pub fn update(&mut self, delta_time: f32) {
        self.velocity *= PARTICLE_VELOCITY_MULTIPLIER;
        self.lifespan = self.lifespan.saturating_sub((LIFESPAN_DECREASE * delta_time) as u8);

        self.velocity += self.acceleration;
        self.pos += self.velocity;

        self.acceleration *= 0.0;

        if self.pos.y >= INTERNAL_RESOLUTION.y || 
           self.pos.y < 0.0 ||
           self.pos.x >= INTERNAL_RESOLUTION.x || 
           self.pos.x < 0.0 
        {
            self.alive = false;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.lifespan != 0 && self.alive
    }

    pub fn show(&self, draw: &mut impl RaylibDraw) {
        draw.draw_circle(
            self.pos.x as i32, 
            self.pos.y as i32, 
            PARTICLE_SIZE, 
            Color { 
                r: self.color.r,
                g: self.color.g,
                b: self.color.b,
                a: self.lifespan
            }
        );
    }
}

#[derive(Debug)]
pub struct Explosion {
    pub pos:   Vector2,
    particles: Vec<Particle>
}

impl Explosion {
    pub fn new(pos: Vector2) -> Self {
        Self {
            pos,
            particles: Vec::with_capacity(MAX_PARTICLE_QTY)
        }
    }

    pub fn explode(&mut self, max_velocity: f32, rainbow: bool, rng: &mut ThreadRng) {
        let amt = rng.random_range(MIN_PARTICLE_QTY..MAX_PARTICLE_QTY);
        for i in 0 .. amt {
            let color = {
                if rainbow {
                    Color::color_from_hsv(i as f32 / amt as f32 * 360.0, 1.0, 1.0)
                } else {
                    FG
                }
            };

            self.particles.push(Particle::new(self.pos, max_velocity, color, rng));
        }
    }

    pub fn explode_with_pos(&mut self, pos: Vector2, max_velocity: f32, rainbow: bool, rng: &mut ThreadRng) {
        self.pos = pos;
        self.explode(max_velocity, rainbow, rng);
    }

    pub fn update(&mut self, delta_time: f32) {
        for particle in self.particles.iter_mut() {
            particle.update(delta_time);
        }

        self.particles.retain(|x| x.is_alive());
    }

    pub fn is_alive(&self) -> bool {
        self.particles.len() != 0
    }

    pub fn show(&self, draw: &mut impl RaylibDraw) {
        for particle in &self.particles {
            particle.show(draw);
        }
    }
}