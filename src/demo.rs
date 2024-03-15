use std::time::{Duration, SystemTime};

use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    fireworks::{Firework, FireworkConfig},
    particle::ParticleConfig,
    utils::{
        explosion_gradient_1, explosion_gradient_2, explosion_gradient_3, gen_points_circle,
        gen_points_circle_normal, linear_gradient_1,
    },
};

pub fn demo_firework_1(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = vec![
        (255, 102, 75),
        (144, 56, 67),
        (255, 225, 124),
        (206, 32, 41),
    ];
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(280., 45).iter() {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(23..27),
            Duration::from_secs_f32(thread_rng().gen_range(2.1..2.7)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default().with_gradient_scale(explosion_gradient_1);
    config.set_enable_gradient(enable_gradient);
    Firework {
        init_time: SystemTime::now(),
        spawn_after,
        center,
        particles,
        config,
        ..Default::default()
    }
}

pub fn demo_firework_2(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = vec![(250, 216, 68)];
    let mut particles = Vec::new();
    for v in gen_points_circle(100, 600).iter() {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(5..8),
            Duration::from_secs_f32(thread_rng().gen_range(3.0..5.5)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_gradient_scale(explosion_gradient_2)
        .with_gravity_scale(0.)
        .with_ar_scale(0.15);
    config.set_enable_gradient(enable_gradient);
    Firework {
        init_time: SystemTime::now(),
        spawn_after,
        center,
        particles,
        config,
        ..Default::default()
    }
}

pub fn demo_firework_3(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = vec![
        (242, 233, 190),
        (226, 196, 136),
        (149, 202, 176),
        (26, 64, 126),
    ];
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(350., 135).iter() {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(23..43),
            Duration::from_secs_f32(thread_rng().gen_range(3.5..5.0)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_gradient_scale(explosion_gradient_1)
        .with_ar_scale(0.18)
        .with_gravity_scale(0.7);
    config.set_enable_gradient(enable_gradient);
    Firework {
        init_time: SystemTime::now(),
        spawn_after,
        center,
        particles,
        config,
        ..Default::default()
    }
}

pub fn demo_firework_4(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = vec![(242, 233, 190), (226, 196, 136), (255, 248, 253)];
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(350., 25).iter() {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(20..33),
            Duration::from_secs_f32(thread_rng().gen_range(3.5..5.0)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_gradient_scale(explosion_gradient_1)
        .with_gravity_scale(0.3);
    config.set_enable_gradient(enable_gradient);
    Firework {
        init_time: SystemTime::now(),
        spawn_after,
        center,
        particles,
        config,
        ..Default::default()
    }
}

pub fn demo_firework_5(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = vec![(152, 186, 227), (54, 84, 117), (21, 39, 60)];
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(450., 80).iter() {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(33..43),
            Duration::from_secs_f32(thread_rng().gen_range(3.5..5.0)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_gradient_scale(explosion_gradient_3)
        .with_gravity_scale(1.4);
    config.set_enable_gradient(enable_gradient);
    Firework {
        init_time: SystemTime::now(),
        spawn_after,
        center,
        particles,
        config,
        ..Default::default()
    }
}

pub fn demo_firework_6(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = vec![(242, 233, 190), (226, 196, 136), (255, 248, 253)];
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(350., 35).iter() {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(20..23),
            Duration::from_secs_f32(thread_rng().gen_range(3.5..4.0)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_gradient_scale(explosion_gradient_1)
        .with_ar_scale(0.19)
        .with_gravity_scale(0.1);
    config.set_enable_gradient(enable_gradient);
    Firework {
        init_time: SystemTime::now(),
        spawn_after,
        center,
        particles,
        config,
        ..Default::default()
    }
}

pub fn demo_firework_comb_1(
    start: Vec2,
    spawn_after: Duration,
    enable_gradient: bool,
) -> Vec<Firework> {
    // Ascent of rocket
    let color1 = (255, 255, 235);
    let particles1 = ParticleConfig::new(
        start,
        Vec2::NEG_Y * 160.,
        6,
        Duration::from_secs_f32(1.2),
        color1,
    );
    let mut config1 = FireworkConfig::default()
        .with_ar_scale(0.04)
        .with_gradient_scale(linear_gradient_1);
    config1.set_enable_gradient(enable_gradient);

    // Explosion
    let color2 = vec![
        (235, 39, 155),
        (250, 216, 68),
        (242, 52, 72),
        (63, 52, 200),
        (255, 139, 57),
    ];
    let center2 = start + Vec2::NEG_Y * 53.;
    let mut particles2 = Vec::new();
    for v in gen_points_circle_normal(350., 160).iter() {
        particles2.push(ParticleConfig::new(
            center2,
            *v,
            thread_rng().gen_range(23..43),
            Duration::from_secs_f32(thread_rng().gen_range(2.5..4.5)),
            *color2.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config2 = FireworkConfig::default()
        .with_gradient_scale(explosion_gradient_1)
        .with_ar_scale(0.2)
        .with_gravity_scale(0.3);
    config2.set_enable_gradient(enable_gradient);
    vec![
        Firework {
            init_time: SystemTime::now(),
            spawn_after,
            center: start,
            particles: vec![particles1],
            config: config1,
            ..Default::default()
        },
        Firework {
            init_time: SystemTime::now(),
            spawn_after: spawn_after + Duration::from_secs_f32(1.2),
            center: center2,
            particles: particles2,
            config: config2,
            ..Default::default()
        },
    ]
}

pub fn demo_firework_comb_0(
    center: Vec2,
    spawn_after: Duration,
    enable_gradient: bool,
) -> Vec<Firework> {
    let mut res = Vec::new();
    res.push(demo_firework_3(
        center + Vec2::new(-5., -19.),
        spawn_after,
        enable_gradient,
    ));
    res.push(demo_firework_4(
        center + Vec2::new(-30., 0.),
        spawn_after + Duration::from_secs_f32(0.4),
        enable_gradient,
    ));
    res.push(demo_firework_5(
        center + Vec2::new(12., 0.),
        spawn_after + Duration::from_secs_f32(1.6),
        enable_gradient,
    ));
    res.push(demo_firework_1(
        center + Vec2::new(-9., 7.),
        spawn_after + Duration::from_secs_f32(2.),
        enable_gradient,
    ));
    res.push(demo_firework_6(
        center + Vec2::new(24., -11.),
        spawn_after + Duration::from_secs_f32(2.3),
        enable_gradient,
    ));
    res
}
