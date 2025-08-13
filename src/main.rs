use std::{cell::OnceCell, cmp::min, collections::HashMap, time::Instant};

use pad::Pad;
use player::Player;
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use raylib::{audio::{RaylibAudio, Sound, SoundAlias}, color::Color, ffi::{KeyboardKey, MouseButton, TraceLogLevel}, math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibTextureModeExt}, texture::{Image, RenderTexture2D}, window::{get_current_monitor, get_monitor_refresh_rate}, RaylibHandle};

use crate::{bomb::Bomb, explosion::Explosion, obstacle::{explosion::ObstacleExplosion, rock::Rock, rocket::{self, Rocket}, AnyObstacle, Obstacle}, obstacle_grid::ObstacleGrid, utils::vec2};

const INTERNAL_RESOLUTION: Vector2 = Vector2 { x: 320.0, y: 180.0 };

const COLLISION_TEST: bool = false; // default: false
const START_OBSTACLES_EARLY: bool = COLLISION_TEST || false; // default: false
const BOMB_TEST: bool = false; // default: false
const ROCKETS: bool = true; // default: true

const INTRO_TEXT: &str = "JUMP TO START";
const INTRO_TEXT_HEIGHT: i32 = 10;
const INTRO_TEXT_Y_OFFSET: i32 = 25;
const SCORE_TEXT_HEIGHT: i32 = 10;

const HIT_SOUND:    &[u8] = include_bytes!("../resources/hit.wav");
const DEATH_SOUND:  &[u8] = include_bytes!("../resources/death.wav");
const BOMB_SOUND:   &[u8] = include_bytes!("../resources/bomb.wav");
const ROCKET_SOUND: &[u8] = include_bytes!("../resources/rocket.wav");
const PEW_SOUND:    &[u8] = include_bytes!("../resources/pew.wav");
const SOUND_EXT: &str = ".wav";

const LIGHTNING: &[u8] = include_bytes!("../resources/lightning.png");
const LIGHTNING_EXT: &str = ".png";

const FG: Color = Color::WHITE;
const BG: Color = Color::BLACK;

const GRAVITY: Vector2 = Vector2 { x: 0.0, y: 0.15 };
const PAD_SIZE: Vector2 = Vector2 { x: 4.0, y: 25.0 };
const PLAYER_VELOCITY: Vector2 = Vector2 { x: 1.25, y: 0.0 };
const OBSTACLE_SAFE_ZONE: Vector2 = Vector2 { x: 70.0, y: 20.0 };
const SPRINT_LINE_POS: Vector2 = Vector2 { x: 10.0, y: 10.0 };
const LIGHTNING_LINE_RPOS: Vector2 = Vector2 { x: 5.0, y: 6.5 };
const NO_OBSTACLES_CENTER_ZONE: Vector2 = Vector2 { x: 50.0, y: 50.0 };
const LIGHTNING_POS: Vector2 = Vector2 { x: SPRINT_LINE_POS.x + SPRINT_LINE_MAX_LENGTH + LIGHTNING_LINE_RPOS.x, y: LIGHTNING_LINE_RPOS.y };

const PLAYER_SIZE: f32 = 5.0;
const PAD_WALL_DISTANCE: f32 = 10.0;
const REFERENCE_FRAMERATE: f32 = 60.0;
const PAD_MOVE_SPEED_MLT: f32 = 3.0;
const DEFAULT_TOLERANCE: f32 = 4.0;
const ALPHA_CHANGE: u8 = 100;
const LIFESPAN_DECREASE: f32 = 3.0;
const PARTICLE_SIZE: f32 = 1.0;
const MIN_PARTICLE_QTY: usize = 25;
const MAX_PARTICLE_QTY: usize = 50;
const MIN_OBSTACLE_SIZE: f32 = 4.0;
const MAX_OBSTACLE_SIZE: f32 = 10.0;
const MIN_OBSTACLE_LIFE: f32 = 40.0;
const MAX_OBSTACLE_LIFE: f32 = 900.0;
const OBSTACLE_START_ALPHA: u8 = 30;
const OBSTACLE_DELTA_ALPHA: i8 = 5;
const PLAYER_COUNT_OBST: u64 = if START_OBSTACLES_EARLY { 0 } else { 10 };
const MOD_INCREMENT_DIFF: u64 = 5;
const MIN_ROCKET_SPEED: f32 = 1.75;
const MAX_ROCKET_SPEED: f32 = 3.5;
const START_DIFFICULTY: u16 = if START_OBSTACLES_EARLY { 1 } else { 0 };
const BOMB_SIZE: f32 = 6.0;
const BOMB_LIFE: f32 = 150.0;
const PLAYER_COUNT_BOMB: u64 = if BOMB_TEST { 0 } else { 20 };
const SPRINT_CHARGE_DELTA: f32 = 1.0;
const SPRINT_MAX_VALUE: f32 = 900.0;
const SPRINT_USE_DELTA: f32 = 10.0;
const SPRINT_VELOCITY: f32 = 4.0;
const SPRINT_ALPHA_CHANGE: u8 = 40;
const SHAKE: f32 = 4.0;
const SPRINT_LINE_MIN_LENGTH: f32 = 2.0;
const SPRINT_LINE_MAX_LENGTH: f32 = 50.0;
const SPRINT_LINE_WIDTH: f32 = 1.0;
const SPRINT_COOLDOWN: f32 = 30.0;
const HIT_COOLDOWN: f32 = 5.0;
const LIGHTNING_SIZE: i32 = 7;
const JUMP_VELOCITY: f32 = 2.5;
const RAINBOW_DELTA: f32 = 0.01;
const HOVER_SPACE: f32 = 8.0;
const HOVER_RAINBOW_DELTA: f32 = 0.001;
const HOVER_RAINBOW_DISTANCE: f32 = 0.05;
const DEATH_MAX_INIT_PARTICLE_VELOCITY: f32 = 1.75;
const PARTICLE_VELOCITY_MULTIPLIER: f32 = 0.98;
const OBSTACLE_GRID_DIV_X: f32 = 12.0;
const OBSTACLE_GRID_DIV_Y: f32 = 6.0;
const OBSTACLE_POS_VARIANCE: f32 = 6.0;
const ROCKET_SHAKE: f32 = 1.0;
const OBSTACLE_COLLISION_MAX_VELOCITY: f32 = 0.8;
const OBSTACLE_PROBABILITY: u16 = if COLLISION_TEST { 1 } else { 1000 };
const BOMB_ANGLE_INCREMENT: f32 = 0.05;
const BOMB_PROBABILITY: u16 = if BOMB_TEST { 0 } else { 2500 };
const BOMB_MIN_DESTROYED_OBSTACLES: usize = 1;
const BOMB_MAX_DESTROYED_OBSTACLES: usize = 4;
const SCORE_HITBOX_SIZE: f32 = 2.0;

const EFFECTIVE_PAD_FRMT: f32 = REFERENCE_FRAMERATE / PAD_MOVE_SPEED_MLT;
const ASPECT_RATIO_H: f32 = INTERNAL_RESOLUTION.x / INTERNAL_RESOLUTION.y;
const ASPECT_RATIO_W: f32 = INTERNAL_RESOLUTION.y / INTERNAL_RESOLUTION.x;
const REFERENCE_FRAMETIME: f32 = 1.0 / REFERENCE_FRAMERATE;

mod utils;
mod explosion;
mod player;
mod pad;
mod obstacle;
mod dither;
mod obstacle_grid;
mod bomb;

#[derive(Debug)]
struct NotPong {
    player: Player,
    
    left_pad:  Pad,
    right_pad: Pad,

    obstacles: Vec<AnyObstacle>,
    obstacle_grid: ObstacleGrid,
    bomb: Option<Bomb>,

    difficulty: u16,
    last_player_count: u64,

    custom_pad_count: u32,
    sprint_amount: f32,

    alpha_change: u8,

    sprint_cooldown: f32,
    hit_cooldown: f32,

    rng: ThreadRng,
    frame_n: u64,

    curr_rocket_id: u16,
}

impl NotPong {
    pub fn new() -> Self {
        Self {
            left_pad: Pad::default(true),
            right_pad: Pad::default(false),
            player: Player::new(),
            obstacles: Vec::new(),
            obstacle_grid: ObstacleGrid::new(),
            bomb: None,
            difficulty: START_DIFFICULTY,
            last_player_count: 0,
            custom_pad_count: 0,
            sprint_amount: 0.0,
            alpha_change: ALPHA_CHANGE,
            sprint_cooldown: SPRINT_COOLDOWN,
            hit_cooldown: HIT_COOLDOWN,
            rng: rand::rng(),
            frame_n: 0,
            curr_rocket_id: 0
        }
    }

    fn reset_part(&mut self) {
        self.curr_rocket_id = 0;
        self.difficulty = START_DIFFICULTY;
        self.last_player_count = 0;
        self.obstacles.clear();
        self.bomb.take();
        self.custom_pad_count = 0;
        self.sprint_amount = 0.0;
        self.sprint_cooldown = SPRINT_COOLDOWN;
        self.hit_cooldown = HIT_COOLDOWN;
        self.obstacle_grid.reset();
    }

    fn player_sprint_off(&mut self) {
        self.player.sprint_off();
        self.alpha_change = ALPHA_CHANGE;
    }

    fn player_sprint_on(&mut self) {
        if self.sprint_amount > 0.0 && self.sprint_cooldown >= SPRINT_COOLDOWN {
            self.sprint_cooldown = 0.0;
            self.player.sprint_on();
            self.alpha_change = SPRINT_ALPHA_CHANGE;
        }
    }

    fn reset(&mut self) {
        self.reset_part();
        self.left_pad.reset();
        self.right_pad.reset();
        self.player_sprint_off();
    }

    fn invert(&mut self, hit_sound: &Sound<'_>) {
        if self.hit_cooldown >= HIT_COOLDOWN {
            self.hit_cooldown = 0.0;
            hit_sound.play();
            hit_sound.play();
            hit_sound.play();
            hit_sound.play();
            hit_sound.play();
            self.player.invert();
        }
    }

    fn handle_keys(&mut self, rl: &RaylibHandle) {
        let mut jump = false;
        let mut sprinting = false;

        // TODO: allow swapping left and right
        let width = rl.get_screen_width() as f32;

        for i in 0 .. rl.get_touch_point_count() {
            let pos = rl.get_touch_position(i);
            if pos.x > width / 2.0 {
                jump = true;
            } else {
                sprinting = true;
            }
        }

        if !self.player.sprinting && !self.player.explosion.is_alive() {
            if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) || 
               rl.is_key_pressed(KeyboardKey::KEY_LEFT) || 
               rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT)
            {
                if self.player.playing {
                    self.player_sprint_on();
                }
            } else if jump || rl.is_key_pressed(KeyboardKey::KEY_UP) || 
                      rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            {
                self.player.jump(&mut self.rng);
            }

            return;
        }

        if !sprinting && self.player.playing && self.player.sprinting {
            if rl.is_key_released(KeyboardKey::KEY_RIGHT) || 
               rl.is_key_released(KeyboardKey::KEY_LEFT) || 
               rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) 
            {
                self.player_sprint_off();
            }
        }
    }

    fn new_rocket(&mut self) -> Rocket {
        let base = rocket::Base::random(&mut self.rng);

        let id = self.curr_rocket_id;
        self.curr_rocket_id = self.curr_rocket_id.wrapping_add(1);

        let pos = {
            match base {
                rocket::Base::Left => {
                    if self.rng.random_bool(0.5) && OBSTACLE_SAFE_ZONE.y < self.left_pad.pos.y {
                        Vector2 {
                            x: 0.0,
                            y: self.rng.random_range(OBSTACLE_SAFE_ZONE.y ..= self.left_pad.pos.y)
                        }
                    } else {
                        Vector2 {
                            x: 0.0,
                            y: self.rng.random_range(self.left_pad.pos.y + PAD_SIZE.y ..= INTERNAL_RESOLUTION.y - OBSTACLE_SAFE_ZONE.y)
                        }
                    }
                }
                rocket::Base::Right => {
                    if self.rng.random_bool(0.5) && OBSTACLE_SAFE_ZONE.y < self.right_pad.pos.y {
                        Vector2 {
                            x: INTERNAL_RESOLUTION.x,
                            y: self.rng.random_range(OBSTACLE_SAFE_ZONE.y ..= self.right_pad.pos.y)
                        }
                    } else {
                        Vector2 {
                            x: INTERNAL_RESOLUTION.x,
                            y: self.rng.random_range(self.right_pad.pos.y + PAD_SIZE.y ..= INTERNAL_RESOLUTION.y - OBSTACLE_SAFE_ZONE.y)
                        }
                    }
                }
                rocket::Base::Top => {
                    Vector2 {
                        x: self.rng.random_range(OBSTACLE_SAFE_ZONE.x ..= INTERNAL_RESOLUTION.x - OBSTACLE_SAFE_ZONE.x),
                        y: 0.0
                    }
                }
                rocket::Base::Bottom => {
                    Vector2 {
                        x: self.rng.random_range(OBSTACLE_SAFE_ZONE.x ..= INTERNAL_RESOLUTION.x - OBSTACLE_SAFE_ZONE.x),
                        y: INTERNAL_RESOLUTION.y
                    }
                }
            }
        };

        Rocket::new(&mut self.rng, id, pos, base)
    }

    fn make_rocket<'a>(&mut self, rocket_sounds: &mut HashMap<u16, SoundAlias<'a, 'a>>, rocket_sound: &'a Sound<'a>) {
        let rocket = self.new_rocket();
        let sound = rocket_sound.alias().expect("Could not alias sound");
        sound.play();
        rocket_sounds.insert(rocket.id, sound);
        self.obstacles.push(rocket.into());
    }

    fn get_alpha_change(&self, delta_time: f32) -> u8 {
        min((self.alpha_change as f32 * delta_time) as u8, 255)
    }

    fn run(&mut self) {
        let (mut rl, thread) = raylib::init()
            .title("!pong")
            .size(1280, 720)
            .resizable()
            .build();
        
        let audio = RaylibAudio::init_audio_device()
            .expect("Could not initialize audio");

        let hit_sound = audio.new_sound_from_wave(
            &audio.new_wave_from_memory(SOUND_EXT, HIT_SOUND)
                .expect("Could not load sound")
        ).expect("Could not load sound");

        let death_sound = audio.new_sound_from_wave(
            &audio.new_wave_from_memory(SOUND_EXT, DEATH_SOUND)
                .expect("Could not load sound")
        ).expect("Could not load sound");

        let bomb_sound = audio.new_sound_from_wave(
            &audio.new_wave_from_memory(SOUND_EXT, BOMB_SOUND)
                .expect("Could not load sound")
        ).expect("Could not load sound");

        let rocket_sound = audio.new_sound_from_wave(
            &audio.new_wave_from_memory(SOUND_EXT, ROCKET_SOUND)
                .expect("Could not load sound")
        ).expect("Could not load sound");

        let pew_sound = audio.new_sound_from_wave(
            &audio.new_wave_from_memory(SOUND_EXT, PEW_SOUND)
                .expect("Could not load sound")
        ).expect("Could not load sound");

        let lightning = {
            let mut image = Image::load_image_from_mem(LIGHTNING_EXT, LIGHTNING)
                .expect("Could not load image");

            image.resize_nn(LIGHTNING_SIZE, LIGHTNING_SIZE);

            rl.load_texture_from_image(&thread, &image)
                .expect("Could not load texture")
        };

        rl.set_target_fps(get_monitor_refresh_rate(get_current_monitor()) as u32);
        rl.set_trace_log(TraceLogLevel::LOG_NONE);

        let mut texture = OnceCell::from(
            rl.load_render_texture(
                &thread, 
                INTERNAL_RESOLUTION.x as u32,
                INTERNAL_RESOLUTION.y as u32,
            ).expect("Could not load render texture")
        );

        self.player.init(&mut self.rng);

        let mut rocket_sounds: HashMap<u16, SoundAlias<'_, '_>> = HashMap::new();
        let mut last_logic_update = Instant::now();

        while !rl.window_should_close() {
            let delta_time = rl.get_frame_time() * REFERENCE_FRAMERATE;
            let should_update_logic = last_logic_update.elapsed().as_secs_f32() >= REFERENCE_FRAMETIME;
            if should_update_logic {
                last_logic_update = Instant::now();
            }

            let tolerance = {
                if delta_time > 1.0 {
                    DEFAULT_TOLERANCE * delta_time
                } else {
                    DEFAULT_TOLERANCE
                }
            };

            self.handle_keys(&rl);
            let mut draw = rl.begin_texture_mode(&thread, get_expect_mut!(texture));

            if self.player.playing {
                draw.draw_text(
                    &self.player.count.to_string(), 
                    (INTERNAL_RESOLUTION.x / 2.0) as i32, (INTERNAL_RESOLUTION.y / 2.0) as i32, 
                    SCORE_TEXT_HEIGHT,
                    FG
                );
            }

            draw.draw_rectangle(
                0, 0, 
                INTERNAL_RESOLUTION.x as i32, INTERNAL_RESOLUTION.y as i32,  
                Color { r: BG.r, g: BG.g, b: BG.b, a: self.get_alpha_change(delta_time) }
            );

            if self.player.is_dead(&mut self.rng) {
                for (_, sound) in rocket_sounds.drain() {
                    sound.stop();
                }

                death_sound.play();
                self.reset();
            }

            if self.player.playing {
                draw.draw_rectangle(
                    SPRINT_LINE_POS.x as i32, 
                    SPRINT_LINE_POS.y as i32, 
                    (SPRINT_LINE_MIN_LENGTH + self.sprint_amount * SPRINT_LINE_MAX_LENGTH / SPRINT_MAX_VALUE) as i32, 
                    SPRINT_LINE_WIDTH as i32, 
                    FG
                );

                draw.draw_texture(
                    &lightning,
                    LIGHTNING_POS.x as i32,
                    LIGHTNING_POS.y as i32,
                    Color::WHITE
                );

                self.left_pad.update(delta_time, &mut draw);
                self.right_pad.update(delta_time, &mut draw);

                if self.left_pad.collides(self.player.pos, tolerance, &mut self.rng) || self.right_pad.collides(self.player.pos, tolerance, &mut self.rng) {
                    self.invert(&hit_sound);
                }

                if let Some(bomb) = &mut self.bomb {
                    if bomb.is_alive() {
                        bomb.update(delta_time, &mut draw);

                        if bomb.to_destroy.is_empty() && bomb.collides(self.player.pos, vec2(PLAYER_SIZE, PLAYER_SIZE)) {
                            bomb_sound.play();

                            // select `amount` random rocks' positions to destroy them
                            let amount = self.rng.random_range(BOMB_MIN_DESTROYED_OBSTACLES..=BOMB_MAX_DESTROYED_OBSTACLES);
                            let mut to_destroy: Vec<Rectangle> = self.obstacles.iter()
                                .filter(|x| matches!(x, AnyObstacle::Rock(_)))
                                .choose_multiple(&mut self.rng, amount)
                                .into_iter()
                                .map(|obstacle| {
                                    let pos = obstacle.pos();
                                    let size = obstacle.size();
                                    Rectangle { x: pos.x, y: pos.y, width: size.x, height: size.y }
                                })
                                .collect();
                            
                            // if there are no obstacles to destroy, the bomb will give points instead
                            if to_destroy.is_empty() {
                                bomb.give_points = amount as u64;
                                bomb.to_destroy.push(Rectangle { 
                                    x: INTERNAL_RESOLUTION.x / 2.0 - SCORE_HITBOX_SIZE / 2.0, 
                                    y: INTERNAL_RESOLUTION.y / 2.0 - SCORE_HITBOX_SIZE / 2.0, 
                                    width:  SCORE_HITBOX_SIZE, 
                                    height: SCORE_HITBOX_SIZE
                                });
                            } else {
                                bomb.to_destroy.append(&mut to_destroy);
                            }
                        }
                    } else {
                        if bomb.give_points > 0 {
                            pew_sound.play();
                            self.player.count += bomb.give_points;
                            let mut explosion = Explosion::new(bomb.pos);
                            explosion.explode(OBSTACLE_COLLISION_MAX_VELOCITY, true, &mut self.rng);
                            self.obstacles.push(ObstacleExplosion(explosion).into());
                        }

                        self.bomb.take();
                    }
                } else if self.player.count >= PLAYER_COUNT_BOMB {
                    if should_update_logic {
                        if self.rng.random_range(0..=BOMB_PROBABILITY) < 1 {
                            // TODO: we should probably avoid the possibility of spawning the bomb directly on the player, 
                            //       but it's an advantage for them so it's fine for now
                            self.bomb = Some(Bomb::new(Vector2 { 
                                x: self.rng.random_range(OBSTACLE_SAFE_ZONE.x ..= INTERNAL_RESOLUTION.x - OBSTACLE_SAFE_ZONE.x), 
                                y: self.rng.random_range(OBSTACLE_SAFE_ZONE.y ..= INTERNAL_RESOLUTION.y - OBSTACLE_SAFE_ZONE.y), 
                            }));
                        } 
                    }
                }

                if self.player.count >= PLAYER_COUNT_OBST {
                    if self.player.count != self.last_player_count && self.player.count % MOD_INCREMENT_DIFF == 0 {
                        self.last_player_count = self.player.count;
                        self.difficulty += 1;
                    }

                    if should_update_logic {
                        if self.rng.random_bool(0.5) {
                            if self.rng.random_range(0..=OBSTACLE_PROBABILITY) < self.difficulty {
                                if let Some((id, pos)) = self.obstacle_grid.alloc(self.player.pos, &mut self.rng) {
                                    self.obstacles.push(Rock::new(&mut self.rng, id, pos).into());
                                } else if ROCKETS { // if you can't allocate a rock, make a rocket instead
                                    self.make_rocket(&mut rocket_sounds, &rocket_sound);
                                }
                            }
                        } else {
                            if ROCKETS {
                                if self.rng.random_range(0..=OBSTACLE_PROBABILITY) < self.difficulty {
                                    self.make_rocket(&mut rocket_sounds, &rocket_sound);
                                }
                            }
                        }
                    }
                }

                for i in 0 .. self.obstacles.len() {
                    self.obstacles[i].update(delta_time, &mut self.rng, &mut draw);

                    if !self.obstacles[i].can_collide() {
                        continue;
                    }

                    if let Some(bomb) = &mut self.bomb {
                        if self.obstacles[i].collides_object(bomb.pos, vec2(BOMB_SIZE, BOMB_SIZE)) {
                            pew_sound.play();
                            self.obstacles[i].kill();
                            let mut explosion = Explosion::new(self.obstacles[i].pos());
                            explosion.explode(OBSTACLE_COLLISION_MAX_VELOCITY, true, &mut self.rng);
                            self.obstacles.push(ObstacleExplosion(explosion).into());
                            continue;
                        }
                    }
                    
                    if !COLLISION_TEST {
                        if self.obstacles[i].collides_object(self.player.pos, vec2(PLAYER_SIZE, PLAYER_SIZE)) {
                            self.player.dead = true;
                            break;
                        }
                    }

                    if let AnyObstacle::Rocket(rocket) = &self.obstacles[i] {
                        rocket_sounds.get(&rocket.id).unwrap()
                            .set_pan(1.0 - rocket.pos.x / INTERNAL_RESOLUTION.x);
                    }

                    for j in 0 .. self.obstacles.len() {
                        if j == i {
                            continue;
                        }

                        if !self.obstacles[j].can_collide() {
                            continue;
                        }

                        let collides = {
                            if i < j {
                                let (l, r) = self.obstacles.split_at_mut(j);
                                l[i].collides_other(&r[0])
                            } else {
                                let (l, r) = self.obstacles.split_at_mut(i);
                                l[j].collides_other(&r[0])
                            }
                        };

                        if collides {
                            self.obstacles[i].kill();
                            self.obstacles[j].kill();
                            let mut explosion = Explosion::new(self.obstacles[i].pos());
                            explosion.explode(OBSTACLE_COLLISION_MAX_VELOCITY, true, &mut self.rng);
                            self.obstacles.push(ObstacleExplosion(explosion).into());
                        }
                    }
                }

                self.obstacles.retain_mut(|obstacle| {
                    if obstacle.is_alive() {
                        return true;
                    }

                    match obstacle {
                        AnyObstacle::Rock(rock) => {
                            self.obstacle_grid.free(rock.id).expect("Same ID was freed twice");
                        }
                        AnyObstacle::Rocket(rocket) => {
                            rocket_sounds.remove(&rocket.id).expect("Rocket sound wasn't in map").stop();
                        }
                        _ => ()
                    }
                    
                    false
                });

                if self.player.sprinting {                    
                    if self.sprint_amount > 0.0 {
                        self.sprint_amount -= SPRINT_USE_DELTA * delta_time;
                    } else {
                        self.player_sprint_off();
                    }
                } else {
                    if self.sprint_amount < SPRINT_MAX_VALUE {
                        self.sprint_amount += SPRINT_CHARGE_DELTA * delta_time;
                    }
                }

                self.sprint_cooldown += delta_time;
                self.hit_cooldown += delta_time;
            } else if !self.player.explosion.is_alive() {
                let size = draw.measure_text(INTRO_TEXT, INTRO_TEXT_HEIGHT);
                draw.draw_text(
                    INTRO_TEXT, 
                    INTERNAL_RESOLUTION.x as i32 / 2 - size / 2, 
                    INTERNAL_RESOLUTION.y as i32 / 2 - INTRO_TEXT_HEIGHT / 2 - INTRO_TEXT_Y_OFFSET, 
                    INTRO_TEXT_HEIGHT, 
                    FG
                );
            }

            self.player.update(delta_time, &mut self.rng, &mut draw);

            drop(draw);

            let raw_texture = texture.take().unwrap().to_raw();
            let mut image = unsafe { Image::from_raw(raylib::ffi::LoadImageFromTexture(raw_texture.texture)) };
            texture.set(unsafe { RenderTexture2D::from_raw(raw_texture) }).unwrap();

            dither::apply(&mut image);
            let postprocessed = rl.load_texture_from_image(&thread, &image)
                .expect("Couldn't load postprocessed image");

            let mut destination = {
                let width = rl.get_screen_width() as f32;
                let height = rl.get_screen_height() as f32;

                let custom_width = height * ASPECT_RATIO_H;
                let custom_height = width * ASPECT_RATIO_W;

                if width > custom_width {
                    Rectangle { 
                        x: width / 2.0 - custom_width / 2.0,
                        y: 0.0,
                        width: custom_width,
                        height
                    }
                } else {
                    Rectangle { 
                        x: 0.0,
                        y: height / 2.0 - custom_height / 2.0,
                        width,
                        height: custom_height
                    }
                }
            };

            let bounding_box = destination;

            if self.player.sprinting {
                destination.x += self.rng.random_range(-SHAKE..=SHAKE);
                destination.y += self.rng.random_range(-SHAKE..=SHAKE);
            }
            
            let mut draw = rl.begin_drawing(&thread);
            draw.clear_background(Color::BLACK);
            // draws the texture flipped upside down (coordinate system is y-flipped in texture mode)
            draw.draw_texture_pro( 
                &postprocessed, 
                Rectangle { x: 0.0, y: 0.0, width: INTERNAL_RESOLUTION.x, height: -INTERNAL_RESOLUTION.y }, 
                destination, 
                Vector2 { x: 0.0, y: 0.0 }, 
                0.0, Color::WHITE
            );

            // game box
            draw.draw_rectangle_lines(
                bounding_box.x as i32, bounding_box.y as i32, 
                bounding_box.width as i32 + 1, bounding_box.height as i32 + 1, 
                Color::BLUEVIOLET
            );

            self.frame_n = self.frame_n.wrapping_add(1);
        }
    }
}

fn main() {
    NotPong::new().run()
}