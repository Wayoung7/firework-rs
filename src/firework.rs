use std::time::{Duration, SystemTime};

use glam::Vec2;

use crate::particle::{LifeState, Particle};

pub struct Firework {
    pub init_time: SystemTime,
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

    pub fn is_gone(&self) -> bool {
        self.state == FireworkState::Gone
    }

    pub fn reset(&mut self) {
        self.init_time = SystemTime::now();
        self.state = FireworkState::Waiting;
        for ele in self.particles.iter_mut() {
            ele.reset();
        }
    }
}

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

pub struct FireworkConfig {
    pub gravity_scale: f32,
    pub ar_scale: f32,
    pub additional_force: Vec2,
    pub gradient_scale: fn(f32) -> f32,
}

impl Default for FireworkConfig {
    fn default() -> Self {
        Self {
            gravity_scale: 1.,
            ar_scale: 0.28,
            additional_force: Vec2::ZERO,
            gradient_scale: |_| 1.,
        }
    }
}

impl FireworkConfig {
    #[inline]
    #[must_use]
    pub fn with_gradient_scale(mut self, f: fn(f32) -> f32) -> Self {
        self.gradient_scale = f;
        self
    }

    #[inline]
    #[must_use]
    pub fn with_gravity_scale(mut self, s: f32) -> Self {
        self.gravity_scale = s;
        self
    }

    #[inline]
    #[must_use]
    pub fn with_ar_scale(mut self, s: f32) -> Self {
        self.ar_scale = s;
        self
    }

    #[inline]
    #[must_use]
    pub fn with_additional_force(mut self, af: Vec2) -> Self {
        self.additional_force = af;
        self
    }
}

pub struct FireworkManager {
    pub fireworks: Vec<Firework>,
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
    pub fn new(fireworks: Vec<Firework>) -> Self {
        Self {
            fireworks,
            enable_loop: false,
        }
    }

    #[inline]
    #[must_use]
    pub fn add_firework(mut self, firework: Firework) -> Self {
        self.fireworks.push(firework);
        self
    }

    #[inline]
    #[must_use]
    pub fn add_fireworks(mut self, mut fireworks: Vec<Firework>) -> Self {
        self.fireworks.append(&mut fireworks);
        self
    }

    #[inline]
    #[must_use]
    pub fn enable_loop(mut self) -> Self {
        self.enable_loop = true;
        self
    }

    #[inline]
    #[must_use]
    pub fn disable_loop(mut self) -> Self {
        self.enable_loop = false;
        self
    }

    pub fn reset(&mut self) {
        for ele in self.fireworks.iter_mut() {
            ele.reset();
        }
    }

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

// pub struct Firework<'a> {
//     pub timer: Duration,
//     pub triggered_time: Option<Duration>,
//     pub trigger: Vec<(fn(&Firework) -> bool, &'a mut Firework<'a>)>,
//     pub center: Vec2,
//     pub particle_gen: fn() -> Vec<Particle>,
//     pub force_field: fn(Vec2, Vec2) -> Vec2,
//     pub state: FireworkState,
// }

// impl<'a> Default for Firework<'a> {
//     fn default() -> Self {
//         Self {
//             timer: Duration::ZERO,
//             triggered_time: None,
//             trigger: Vec::new(),
//             center: Vec2::ZERO,
//             particle_gen: || Vec::new(),
//             force_field: |_, _| Vec2::ZERO,
//             state: FireworkState::Waiting,
//         }
//     }
// }

// impl<'a> Firework<'a> {
//     pub fn with_center(&mut self, center: Vec2) {
//         self.center = center;
//     }
//     pub fn with_trigger(
//         &mut self,
//         trigger: &mut Vec<(fn(&Firework) -> bool, &'a mut Firework<'a>)>,
//     ) {
//         self.trigger.append(trigger);
//     }
//     pub fn with_particle_gen(&mut self, particle_gen: fn() -> Vec<Particle>) {
//         self.particle_gen = particle_gen;
//     }
//     pub fn with_force_field(&mut self, force_field: fn(Vec2, Vec2) -> Vec2) {
//         self.force_field = force_field;
//     }
//     pub fn with_triggered_time(&mut self, triggered_time: Duration) {
//         self.triggered_time = Some(triggered_time);
//     }
//     pub fn spawn(&mut self) {
//         self.state = FireworkState::Alive;
//         (self.particle_gen)();
//     }

//     pub fn update(&mut self, delta_time: Duration) {
//         self.timer += delta_time;
//         if let Some(tt) = self.triggered_time {
//             if tt <= self.timer {
//                 self.state = FireworkState::Alive;
//                 (self.particle_gen)();
//             }
//         }
//         for f in self.trigger.iter_mut() {
//             if (f.0)(&self) {
//                 f.1.spawn();
//             }
//         }
//     }
// }
