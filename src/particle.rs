use std::{collections::HashMap, time::Duration};

use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng};

use crate::{fireworks::FireworkConfig, utils::distance_squared};

/// The struct represents the states in a `Particle`'s lifetime
///
/// Every `Particle` goes from `Alive` -> `Declining` -> `Dying` -> `Dead`
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifeState {
    Alive,
    Declining,
    Dying,
    Dead,
}

/// The struct representing a single particle
#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    /// Records a `trail_length` of previous positions of the `Particle`
    pub trail: Vec<Vec2>,
    pub life_state: LifeState,
    /// Color in RGB (from 0 to 255)
    pub color: (u8, u8, u8),
    /// `Duration` since initialization of this `Particle`
    pub time_elapsed: Duration,
    pub config: ParticleConfig,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            trail: Vec::new(),
            life_state: LifeState::Alive,
            color: (255, 255, 255),
            time_elapsed: Duration::ZERO,
            config: ParticleConfig::default(),
        }
    }
}

impl Particle {
    /// Create a new `Particle`
    pub fn new(
        pos: Vec2,
        vel: Vec2,
        trail_length: usize,
        life_time: Duration,
        color: (u8, u8, u8),
    ) -> Self {
        let mut trail = Vec::with_capacity(trail_length);
        (0..trail_length).for_each(|_| trail.push(pos));
        let life_state = LifeState::Alive;
        Self {
            pos,
            vel,
            trail,
            life_state,
            color,
            time_elapsed: Duration::ZERO,
            config: ParticleConfig::new(pos, vel, trail_length, life_time),
        }
    }

    /// Return true if `Particle`'s `LifeState` is `Dead`
    pub fn is_dead(&self) -> bool {
        self.life_state == LifeState::Dead
    }

    /// Reset `Particle` to its initial state
    pub fn reset(&mut self) {
        self.pos = self.config.init_pos;
        self.vel = self.config.init_vel;
        (0..self.config.trail_length).for_each(|i| self.trail[i] = self.pos);
        self.life_state = LifeState::Alive;
        self.time_elapsed = Duration::ZERO;
    }

    /// Update the `Particle` based on delta time
    ///
    /// # Arguments
    ///
    /// * - `duration` - `Duration` since last update
    pub fn update(&mut self, duration: Duration, config: &FireworkConfig) {
        const TIME_STEP: f32 = 0.001;
        self.time_elapsed += duration;
        self.life_state = cal_life_state(self.config.life_time, self.time_elapsed);
        let mut t = 0.;
        while t < duration.as_secs_f32() {
            self.vel += TIME_STEP
                * (Vec2::Y * 10. * config.gravity_scale
                    - self.vel.normalize() * self.vel.length().powi(2) * config.ar_scale
                    + config.additional_force);
            self.pos += TIME_STEP * self.vel;
            t += TIME_STEP;
        }
        self.trail.remove(0);
        self.trail.push(self.pos);
    }

    /// Return pixel data of the current stage of `Particle`
    pub fn draw(&self, config: &FireworkConfig) -> HashMap<(isize, isize), (char, (u8, u8, u8))> {
        let mut data = HashMap::new();
        let mut v: Vec<Vec2> = self.trail.clone();
        let gradient_scale = if config.enable_gradient {
            Some((config.gradient_scale)(
                self.time_elapsed.as_secs_f32() / self.config.life_time.as_secs_f32(),
            ))
        } else {
            None
        };
        v = v
            .iter()
            .map(|pos| Vec2::new(pos.x * 2., pos.y))
            .collect::<Vec<_>>();
        v.windows(2).enumerate().for_each(|(idx, v)| {
            let path = construct_line(v[0], v[1]);
            set_rand_char(
                &path,
                idx as f32 / self.config.trail_length as f32,
                self.life_state,
                if let Some(g) = gradient_scale {
                    shift_gradient(self.color, g)
                } else {
                    self.color
                },
            )
            .iter()
            .for_each(|(k, v)| {
                data.insert(*k, *v);
            });
        });
        data
    }
}

/// Struct that defines the configuration of `Particle`
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ParticleConfig {
    pub init_pos: Vec2,
    pub init_vel: Vec2,
    pub trail_length: usize,
    /// `Duration` from `Particle`'s initialization to its `Dead`
    pub life_time: Duration,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            init_pos: Vec2::ZERO,
            init_vel: Vec2::ZERO,
            trail_length: 2,
            life_time: Duration::from_secs(3),
        }
    }
}

impl ParticleConfig {
    /// Create a new `ParticleConfig`
    pub fn new(init_pos: Vec2, init_vel: Vec2, trail_length: usize, life_time: Duration) -> Self {
        Self {
            init_pos,
            init_vel,
            trail_length,
            life_time,
        }
    }
}

fn set_rand_char(
    path: &Vec<(isize, isize)>,
    density: f32,
    life_state: LifeState,
    color: (u8, u8, u8),
) -> HashMap<(isize, isize), (char, (u8, u8, u8))> {
    let mut data = HashMap::new();
    path.iter().for_each(|p| {
        if let Some(c) = match life_state {
            LifeState::Alive => Some(get_char_alive(density)),
            LifeState::Declining => Some(get_char_declining(density)),
            LifeState::Dying => Some(get_char_dying(density)),
            LifeState::Dead => None,
        } {
            data.insert(*p, (c, color));
        }
    });
    data
}

fn construct_line(a: Vec2, b: Vec2) -> Vec<(isize, isize)> {
    const STEP: f32 = 0.2;
    let (x0, y0) = (a.x, a.y);
    let (x1, y1) = (b.x, b.y);
    let mut path = Vec::new();
    let mut x = x0;
    let mut y = y0;
    let slope = (y1 - y0) / (x1 - x0);
    let dx = if x0 == x1 {
        0.
    } else if x1 > x0 {
        1.
    } else {
        -1.
    };
    let dy = if y0 == y1 {
        0.
    } else if y1 > y0 {
        1.
    } else {
        -1.
    };
    let mut ds = distance_squared(a, b) + f32::EPSILON;
    path.push((x0.round() as isize, y0.round() as isize));
    if (x1 - x0).abs() >= (y1 - y0).abs() {
        while distance_squared(Vec2::new(x, y), b) <= ds {
            if *path.last().unwrap() != (x.round() as isize, y.round() as isize) {
                path.push((x.round() as isize, y.round() as isize));
                ds = distance_squared(Vec2::new(x, y), b);
            }
            x += dx * STEP;
            y += dy * (STEP * slope).abs();
        }
    } else {
        while distance_squared(Vec2::new(x, y), b) <= ds {
            if *path.last().unwrap() != (x.round() as isize, y.round() as isize) {
                path.push((x.round() as isize, y.round() as isize));
                ds = distance_squared(Vec2::new(x, y), b);
            }
            y += dy * STEP;
            x += dx * (STEP / slope).abs();
        }
    }
    path
}

fn shift_gradient(color: (u8, u8, u8), scale: f32) -> (u8, u8, u8) {
    (
        (color.0 as f32 * scale) as u8,
        (color.1 as f32 * scale) as u8,
        (color.2 as f32 * scale) as u8,
    )
}

fn cal_life_state(life_time: Duration, current_elapsed: Duration) -> LifeState {
    let p = current_elapsed.as_millis() as f32 / life_time.as_millis() as f32;
    if p < 0.4 {
        LifeState::Alive
    } else if p < 0.65 {
        LifeState::Declining
    } else if p < 1. {
        LifeState::Dying
    } else {
        LifeState::Dead
    }
}

fn get_char_alive(density: f32) -> char {
    let palette = if density < 0.3 {
        "`'. "
    } else if density < 0.5 {
        "/\\|()1{}[]?"
    } else if density < 0.7 {
        "oahkbdpqwmZO0QLCJUYXzcvunxrjft*"
    } else {
        "$@B%8&WM#"
    };
    palette.chars().choose(&mut thread_rng()).unwrap()
}

fn get_char_declining(density: f32) -> char {
    let palette = if density < 0.2 {
        "` '. "
    } else if density < 0.6 {
        "-_ +~<> i!lI;:,\"^"
    } else if density < 0.85 {
        "/\\| ()1{}[ ]?"
    } else {
        "xrjft*"
    };
    palette.chars().choose(&mut thread_rng()).unwrap()
}

fn get_char_dying(density: f32) -> char {
    let palette = if density < 0.6 {
        ".  ,`.    ^,' . "
    } else {
        " /\\| ( )  1{} [  ]?i !l I;: ,\"^ "
    };
    palette.chars().choose(&mut thread_rng()).unwrap()
}
