use std::time::{Duration, SystemTime};

use glam::Vec2;

use crate::particle::Particle;

/// Struct representing a single firework
pub struct Firework {
    /// The `SystemTime` when the object is initialized/defined
    pub init_time: SystemTime,
    /// Firework spawns after `spawn_after` from `init_time`
    pub spawn_after: Duration,
    pub center: Vec2,
    pub particles: Vec<Particle>,
    pub state: FireworkState,
    pub config: FireworkConfig,
}

impl Default for Firework {
    fn default() -> Self {
        Self {
            init_time: SystemTime::now(),
            spawn_after: Duration::ZERO,
            center: Vec2::ZERO,
            particles: Vec::new(),
            state: FireworkState::Waiting,
            config: FireworkConfig::default(),
        }
    }
}

impl Firework {
    /// Update the `Firework`
    ///
    /// # Arguments
    ///
    /// * `now` - `SystemTime` of now
    /// * `delta_time` - `Duration` since last update
    pub fn update(&mut self, now: SystemTime, delta_time: Duration) {
        // Spawn particles
        if now >= self.init_time + self.spawn_after {
            self.state = FireworkState::Alive;
        }

        // Update
        if self.state == FireworkState::Alive {
            self.particles
                .iter_mut()
                .for_each(|p| p.update(delta_time, &self.config));
        }

        // Clean the dead pariticles
        // let p = self.particles.clone();
        // self.particles = p
        //     .into_iter()
        //     .filter(|p| p.life_state != LifeState::Dead)
        //     .collect();

        if self.state == FireworkState::Alive
            && self
                .particles
                .iter()
                .fold(true, |acc, x| acc && x.is_dead())
        {
            self.state = FireworkState::Gone;
        }
    }

    /// Return true if the `FireworkState` is `Gone`
    pub fn is_gone(&self) -> bool {
        self.state == FireworkState::Gone
    }

    /// Reset `FireworkManager` to its initial state so that the fireworks show starts again
    pub fn reset(&mut self) {
        self.init_time = SystemTime::now();
        self.state = FireworkState::Waiting;
        for ele in self.particles.iter_mut() {
            ele.reset();
        }
    }
}

/// Struct representing state of a `Firework`
///
/// State goes from `Waiting` -> `Alive` -> `Gone`
///
/// # Notes
///
/// - `Firework` turns to `Alive` when it is spawned
/// - `Firework` turns to `Gone` when all of its `Particles` are `Dead`
#[derive(Debug, PartialEq)]
pub enum FireworkState {
    Waiting,
    Alive,
    Gone,
}

impl Default for FireworkState {
    fn default() -> Self {
        FireworkState::Waiting
    }
}

/// Struct representing the configuration of a single `Firework`
///
/// This applies to all `Particle` in the `Firework`
pub struct FireworkConfig {
    /// Larger `gravity_scale` tends to pull particles down
    pub gravity_scale: f32,
    /// Air resistance scale
    /// Warning: too large or too small `ar_scale` may lead to unexpected behavior of `Particles`
    pub ar_scale: f32,
    pub additional_force: Vec2,
    /// This field is a function that takes a float between 0 and 1, returns a float representing all `Particle`s' gradient
    ///
    /// `Particle`s' gradient changes according to its elapsed time and lifetime
    /// The input `f32` equals to `time_elapsed`/`life_time`, which returns a `f32` affecting its color gradient
    /// `gradient_scale` returns 1. means`Particle` will have the same colors as defined all over its lifetime
    pub gradient_scale: fn(f32) -> f32,
    /// Set wheter or not firework has color gradient
    ///
    /// # Notes
    ///
    /// - It is recommanded that your terminal window is non-transparent and has black bg color to get better visual effects
    /// - Otherwise set it to `false`
    pub enable_gradient: bool,
}

impl Default for FireworkConfig {
    fn default() -> Self {
        Self {
            gravity_scale: 1.,
            ar_scale: 0.28,
            additional_force: Vec2::ZERO,
            gradient_scale: |_| 1.,
            enable_gradient: false,
        }
    }
}

impl FireworkConfig {
    /// Set `gradient_scale`
    #[inline]
    #[must_use]
    pub fn with_gradient_scale(mut self, f: fn(f32) -> f32) -> Self {
        self.gradient_scale = f;
        self
    }

    /// Set `gravity_scale`
    #[inline]
    #[must_use]
    pub fn with_gravity_scale(mut self, s: f32) -> Self {
        self.gravity_scale = s;
        self
    }

    /// Set `ar_scale`
    #[inline]
    #[must_use]
    pub fn with_ar_scale(mut self, s: f32) -> Self {
        self.ar_scale = s;
        self
    }

    /// Set `additional_force`
    #[inline]
    #[must_use]
    pub fn with_additional_force(mut self, af: Vec2) -> Self {
        self.additional_force = af;
        self
    }

    /// Set `enable_gradient`
    pub fn set_enable_gradient(&mut self, enable_gradient: bool) {
        self.enable_gradient = enable_gradient;
    }
}

/// `FireworkManager` manages all `Firework`s
pub struct FireworkManager {
    pub fireworks: Vec<Firework>,
    /// If this is `true`, the whole fireworks show will restart when all the `Firework`s are `Gone`
    pub enable_loop: bool,
}

impl Default for FireworkManager {
    fn default() -> Self {
        Self {
            fireworks: Vec::new(),
            enable_loop: false,
        }
    }
}

impl FireworkManager {
    /// Create a new `FireworkManager` with `enable_loop` set to `false`
    pub fn new(fireworks: Vec<Firework>) -> Self {
        Self {
            fireworks,
            enable_loop: false,
        }
    }

    /// Add a `Firework` to `FireworkManager`
    #[inline]
    #[must_use]
    pub fn add_firework(mut self, firework: Firework) -> Self {
        self.fireworks.push(firework);
        self
    }

    // Add a vector of `Firework`s to `FireworkManager`
    #[inline]
    #[must_use]
    pub fn add_fireworks(mut self, mut fireworks: Vec<Firework>) -> Self {
        self.fireworks.append(&mut fireworks);
        self
    }

    /// Set `enable_loop` to `true`
    #[inline]
    #[must_use]
    pub fn enable_loop(mut self) -> Self {
        self.enable_loop = true;
        self
    }

    /// Set `enable_loop` to `false`
    #[inline]
    #[must_use]
    pub fn disable_loop(mut self) -> Self {
        self.enable_loop = false;
        self
    }

    /// Reset the whole fireworks show
    pub fn reset(&mut self) {
        for ele in self.fireworks.iter_mut() {
            ele.reset();
        }
    }

    pub fn set_enable_loop(&mut self, enable_loop: bool) {
        self.enable_loop = enable_loop;
    }

    /// The main update function
    pub fn update(&mut self, now: SystemTime, delta_time: Duration) {
        for ele in self.fireworks.iter_mut() {
            ele.update(now, delta_time);
        }
        if self.enable_loop {
            if self
                .fireworks
                .iter()
                .fold(true, |acc, x| acc && x.is_gone())
            {
                self.reset();
            }
        }
    }
}
