//! `firework` module provides functions to define, create and update fireworks

use std::time::{Duration, SystemTime};

use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng};

use crate::particle::{LifeState, Particle, ParticleConfig};

/// Struct representing a single firework
pub struct Firework {
    /// The `SystemTime` when the object is initialized/defined
    pub init_time: SystemTime,
    /// Firework spawns after `spawn_after` from `init_time`
    pub spawn_after: Duration,
    pub time_elapsed: Duration,
    pub center: Vec2,
    pub state: FireworkState,
    pub config: FireworkConfig,
    pub form: ExplosionForm,
    pub particles: Vec<ParticleConfig>,
    pub current_particles: Vec<Particle>,
}

impl Default for Firework {
    fn default() -> Self {
        Self {
            init_time: SystemTime::now(),
            spawn_after: Duration::ZERO,
            time_elapsed: Duration::ZERO,
            center: Vec2::ZERO,
            state: FireworkState::Waiting,
            config: FireworkConfig::default(),
            form: ExplosionForm::Instant { used: false },
            particles: Vec::new(),
            current_particles: Vec::new(),
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
            self.time_elapsed += delta_time;
            match &mut self.form {
                ExplosionForm::Instant { used } => {
                    if !*used {
                        self.particles.iter().for_each(|p| {
                            self.current_particles.push(Particle {
                                pos: p.init_pos,
                                vel: p.init_vel,
                                trail: init_trail(p.init_pos, p.trail_length),
                                life_state: LifeState::Alive,
                                time_elapsed: Duration::ZERO,
                                config: *p,
                            })
                        })
                    }
                    *used = true;
                }
                ExplosionForm::Sustained {
                    lasts,
                    time_interval,
                    timer,
                } => {
                    if self.time_elapsed <= *lasts {
                        if *timer + delta_time <= *time_interval {
                            *timer += delta_time;
                        } else {
                            let n =
                                (*timer + delta_time).as_millis() / (*time_interval).as_millis();
                            self.particles
                                .iter()
                                .choose_multiple(&mut thread_rng(), n as usize)
                                .iter()
                                .for_each(|p| {
                                    self.current_particles.push(Particle {
                                        pos: p.init_pos,
                                        vel: p.init_vel,
                                        trail: init_trail(p.init_pos, p.trail_length),
                                        life_state: LifeState::Alive,
                                        time_elapsed: Duration::ZERO,
                                        config: **p,
                                    })
                                });
                            *timer = Duration::from_millis(
                                ((*timer + delta_time).as_millis() % (*time_interval).as_millis())
                                    as u64,
                            );
                        }
                    }
                }
            }
            self.state = FireworkState::Alive;
        }

        self.current_particles
            .iter_mut()
            .for_each(|p| p.update(delta_time, &self.config));

        // Clean the dead pariticles
        self.current_particles
            .retain(|p| p.life_state != LifeState::Dead);

        match self.form {
            ExplosionForm::Instant { used } => {
                if used && self.state == FireworkState::Alive && self.current_particles.is_empty() {
                    self.state = FireworkState::Gone;
                }
            }
            ExplosionForm::Sustained { lasts, .. } => {
                if self.time_elapsed > lasts
                    && self.state == FireworkState::Alive
                    && self.current_particles.is_empty()
                {
                    self.state = FireworkState::Gone;
                }
            }
        }
    }

    /// Return true if the `FireworkState` is `Gone`
    pub fn is_gone(&self) -> bool {
        self.state == FireworkState::Gone
    }

    /// Reset `Firework` to its initial state
    pub fn reset(&mut self) {
        self.init_time = SystemTime::now();
        self.state = FireworkState::Waiting;
        self.time_elapsed = Duration::ZERO;
        self.current_particles = Vec::new();
        match &mut self.form {
            ExplosionForm::Instant { used } => {
                *used = false;
            }
            ExplosionForm::Sustained { timer, .. } => {
                *timer = Duration::ZERO;
            }
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
#[derive(Debug, PartialEq, Default)]
pub enum FireworkState {
    #[default]
    Waiting,
    Alive,
    Gone,
}

/// Enum that represents whether the `Firework` make one instantaneous explosion or continuously emit particles
#[derive(Debug, PartialEq, Eq)]
pub enum ExplosionForm {
    Instant {
        used: bool,
    },
    Sustained {
        /// `Duration` that the sustained firework will last
        lasts: Duration,
        /// Time interval between two particle spawn
        time_interval: Duration,
        timer: Duration,
    },
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
    pub additional_force: Box<dyn Fn(&Particle) -> Vec2>,
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
            additional_force: Box::new(move |_| Vec2::ZERO),
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
    pub fn with_additional_force(mut self, af: impl Fn(&Particle) -> Vec2 + 'static) -> Self {
        self.additional_force = Box::new(af);
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
    /// Controls how fireworks are installed in `FireworkManager`
    pub install_form: FireworkInstallForm,
}

impl Default for FireworkManager {
    fn default() -> Self {
        Self {
            fireworks: Vec::new(),
            enable_loop: false,
            install_form: FireworkInstallForm::StaticInstall,
        }
    }
}

impl FireworkManager {
    /// Create a new `FireworkManager` with `enable_loop` set to `false`
    pub fn new(fireworks: Vec<Firework>) -> Self {
        Self {
            fireworks,
            enable_loop: false,
            install_form: FireworkInstallForm::StaticInstall,
        }
    }

    /// Add a `Firework` to a existing `FireworkManager`
    pub fn add_firework(&mut self, firework: Firework) {
        self.fireworks.push(firework);
    }

    /// Add `Firework`s to a existing `FireworkManager`
    pub fn add_fireworks(&mut self, mut fireworks: Vec<Firework>) {
        self.fireworks.append(&mut fireworks);
    }

    /// Add a `Firework` to `FireworkManager`
    #[inline]
    #[must_use]
    pub fn with_firework(mut self, firework: Firework) -> Self {
        self.fireworks.push(firework);
        self
    }

    // Add a vector of `Firework`s to `FireworkManager`
    #[inline]
    #[must_use]
    pub fn with_fireworks(mut self, mut fireworks: Vec<Firework>) -> Self {
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
        if self.install_form == FireworkInstallForm::DynamicInstall {
            self.fireworks.retain(|f| f.state != FireworkState::Gone);
        }
        if self.install_form == FireworkInstallForm::StaticInstall
            && self.enable_loop
            && self.fireworks.iter().all(|f| f.is_gone())
        {
            self.reset();
        }
    }

    /// Set `install_form` to `DynamicInstall`
    pub fn enable_dyn_install(mut self) -> Self {
        self.install_form = FireworkInstallForm::DynamicInstall;
        self
    }
}

/// `StaticInstall` keeps all the fireworks in `FireworkManager` and won't delete them
///
/// `DynamicInstall` automatically remove fireworks that are `Gone`, which let you add fireworks continuously
///
/// # Notes
///
///  - `FireworkManager` that has `DynamicInstall` can't loop, it will ignore the set `enable_loop` value
#[derive(Debug, PartialEq)]
pub enum FireworkInstallForm {
    StaticInstall,
    DynamicInstall,
}

fn init_trail(init_pos: Vec2, n: usize) -> Vec<Vec2> {
    let mut res = Vec::new();
    (0..n).for_each(|_| res.push(init_pos));
    res
}
