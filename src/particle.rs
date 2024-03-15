//! `particle` module provides functions to define, create and update particles

use std::time::Duration;

use glam::Vec2;

use crate::fireworks::FireworkConfig;

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
            time_elapsed: Duration::ZERO,
            config: ParticleConfig::new(pos, vel, trail_length, life_time, color),
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
                    + (config.additional_force)(self));
            self.pos += TIME_STEP * self.vel;
            t += TIME_STEP;
        }
        self.trail.remove(0);
        self.trail.push(self.pos);
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
    /// Color in RGB (from 0 to 255)
    pub color: (u8, u8, u8),
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            init_pos: Vec2::ZERO,
            init_vel: Vec2::ZERO,
            trail_length: 2,
            life_time: Duration::from_secs(3),
            color: (255, 255, 255),
        }
    }
}

impl ParticleConfig {
    /// Create a new `ParticleConfig`
    pub fn new(
        init_pos: Vec2,
        init_vel: Vec2,
        trail_length: usize,
        life_time: Duration,
        color: (u8, u8, u8),
    ) -> Self {
        Self {
            init_pos,
            init_vel,
            trail_length,
            life_time,
            color,
        }
    }
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
