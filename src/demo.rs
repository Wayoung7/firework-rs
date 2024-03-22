//! This module provides some demos of different types of fireworks

use std::{
    f32::consts::PI,
    time::{Duration, SystemTime},
};

use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng, Rng};

use crate::{
    fireworks::{ExplosionForm, Firework, FireworkConfig},
    particle::ParticleConfig,
    utils::{
        explosion_gradient_1, explosion_gradient_2, explosion_gradient_3, gen_points_arc,
        gen_points_circle, gen_points_circle_normal, gen_points_circle_normal_dev, gen_points_fan,
        linear_gradient_1,
    },
};

pub fn demo_firework_0(
    center: Vec2,
    spawn_after: Duration,
    enable_gradient: bool,
    colors: Vec<(u8, u8, u8)>,
) -> Firework {
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(
        thread_rng().gen_range(230.0..400.0),
        thread_rng().gen_range(33..47),
    )
    .iter()
    {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(20..25),
            Duration::from_secs_f32(thread_rng().gen_range(1.8..2.3)),
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

pub fn demo_firework_1(center: Vec2, spawn_after: Duration, enable_gradient: bool) -> Firework {
    let colors = [
        (255, 102, 75),
        (144, 56, 67),
        (255, 225, 124),
        (206, 32, 41),
    ];
    let mut particles = Vec::new();
    for v in gen_points_circle_normal(250., 45).iter() {
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
    let colors = [(250, 216, 68)];
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
    let colors = [
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
    let colors = [(242, 233, 190), (226, 196, 136), (255, 248, 253)];
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
    let colors = [(152, 186, 227), (54, 84, 117), (21, 39, 60)];
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
    let colors = [(242, 233, 190), (226, 196, 136), (255, 248, 253)];
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
    let color2 = [
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
    vec![
        demo_firework_3(center + Vec2::new(-5., -19.), spawn_after, enable_gradient),
        demo_firework_4(
            center + Vec2::new(-30., 0.),
            spawn_after + Duration::from_secs_f32(0.4),
            enable_gradient,
        ),
        demo_firework_5(
            center + Vec2::new(12., 0.),
            spawn_after + Duration::from_secs_f32(1.6),
            enable_gradient,
        ),
        demo_firework_1(
            center + Vec2::new(-9., 7.),
            spawn_after + Duration::from_secs_f32(2.),
            enable_gradient,
        ),
        demo_firework_6(
            center + Vec2::new(24., -11.),
            spawn_after + Duration::from_secs_f32(2.3),
            enable_gradient,
        ),
    ]
}

pub fn demo_firework_comb_2(
    center: Vec2,
    spawn_after: Duration,
    enable_gradient: bool,
) -> Vec<Firework> {
    let mut res = Vec::new();
    let fountain1 = |center: Vec2, angle: f32| {
        let colors = [(255, 183, 3), (251, 133, 0), (242, 233, 190)];
        let mut particles = Vec::new();
        for v in gen_points_fan(60., 20, angle - 0.05, angle + 0.05).iter() {
            particles.push(ParticleConfig::new(
                center,
                *v,
                thread_rng().gen_range(28..38),
                Duration::from_secs_f32(thread_rng().gen_range(2.5..3.8)),
                *colors.iter().choose(&mut thread_rng()).unwrap(),
            ));
        }
        let mut config = FireworkConfig::default()
            .with_ar_scale(0.05)
            .with_gradient_scale(linear_gradient_1);
        config.set_enable_gradient(enable_gradient);
        Firework {
            init_time: SystemTime::now(),
            spawn_after,
            center,
            particles,
            config,
            form: ExplosionForm::Sustained {
                lasts: Duration::from_secs(5),
                time_interval: Duration::from_secs_f32(0.08),
                timer: Duration::ZERO,
            },
            ..Default::default()
        }
    };

    let fountain2 = |center: Vec2| {
        let colors = [(226, 196, 136), (255, 245, 253), (208, 58, 99)];
        let mut particles = Vec::new();
        for v in gen_points_fan(1000., 20, 5.7 / 12. * PI, 6.3 / 12. * PI).iter() {
            particles.push(ParticleConfig::new(
                center,
                *v,
                thread_rng().gen_range(28..38),
                Duration::from_secs_f32(thread_rng().gen_range(2.5..3.8)),
                *colors.iter().choose(&mut thread_rng()).unwrap(),
            ));
        }
        let mut config = FireworkConfig::default()
            .with_ar_scale(0.14)
            .with_gravity_scale(0.9)
            .with_gradient_scale(linear_gradient_1);
        config.set_enable_gradient(enable_gradient);
        Firework {
            init_time: SystemTime::now(),
            spawn_after: spawn_after + Duration::from_secs_f32(4.),
            center,
            particles,
            config,
            form: ExplosionForm::Sustained {
                lasts: Duration::from_secs(5),
                time_interval: Duration::from_secs_f32(0.08),
                timer: Duration::ZERO,
            },
            ..Default::default()
        }
    };

    let mono = |center: Vec2, sa: Duration, colors: Vec<(u8, u8, u8)>| {
        let particles = vec![ParticleConfig::new(
            center,
            gen_points_arc(200., 1, 5. / 12. * PI, 7. / 12. * PI)[0],
            thread_rng().gen_range(24..30),
            Duration::from_secs_f32(thread_rng().gen_range(2.1..2.7)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        )];
        let mut config = FireworkConfig::default()
            .with_ar_scale(thread_rng().gen_range(0.18..0.24))
            .with_gradient_scale(linear_gradient_1);
        config.set_enable_gradient(enable_gradient);
        Firework {
            init_time: SystemTime::now(),
            spawn_after: sa,
            center,
            particles,
            config,
            form: ExplosionForm::Instant { used: false },
            ..Default::default()
        }
    };

    res.push(fountain1(center + Vec2::new(-31., 21.), 1.05));
    res.push(fountain1(center + Vec2::new(31., 21.), 2.09));

    res.push(fountain2(center + Vec2::new(-7., 21.)));
    res.push(fountain2(center + Vec2::new(7., 21.)));

    (-33..=33).step_by(3).for_each(|i| {
        res.push(mono(
            Vec2::new(center.x + i as f32, center.y + 21.),
            Duration::from_secs_f32(3.5),
            vec![(0, 119, 182), (144, 224, 239), (12, 180, 216)],
        ))
    });

    (-33..=33).step_by(3).for_each(|i| {
        res.push(mono(
            Vec2::new(center.x + i as f32, center.y + 21.),
            Duration::from_secs_f32(4.7),
            vec![(181, 23, 158), (247, 37, 133), (114, 9, 183)],
        ))
    });

    (-33..=33).step_by(3).for_each(|i| {
        res.push(mono(
            Vec2::new(center.x + i as f32, center.y + 21.),
            Duration::from_secs_f32(5.9),
            vec![(217, 237, 146), (153, 217, 140), (82, 182, 154)],
        ))
    });

    res
}

pub fn demo_firework_comb_3(
    center: Vec2,
    spawn_after: Duration,
    enable_gradient: bool,
) -> Vec<Firework> {
    let mut res = Vec::new();
    let f1 = {
        let colors = [(255, 216, 190), (255, 238, 221), (248, 247, 255)];
        let mut particles = Vec::new();
        for v in gen_points_circle_normal_dev(14., 200, 60.).iter() {
            particles.push(ParticleConfig::new(
                center + Vec2::NEG_Y * 6.,
                *v,
                thread_rng().gen_range(15..20),
                Duration::from_secs_f32(thread_rng().gen_range(3.0..5.0)),
                *colors.iter().choose(&mut thread_rng()).unwrap(),
            ));
        }
        let mut config = FireworkConfig::default()
            .with_gradient_scale(explosion_gradient_1)
            .with_ar_scale(0.15)
            .with_gravity_scale(0.35);
        config.set_enable_gradient(enable_gradient);
        Firework {
            init_time: SystemTime::now(),
            spawn_after,
            center,
            particles,
            config,
            ..Default::default()
        }
    };
    res.push(f1);
    let f2 = {
        let colors = [
            (152, 186, 227),
            (89, 129, 177),
            (54, 84, 117),
            (240, 244, 254),
        ];
        let mut particles = Vec::new();
        for v in gen_points_circle_normal_dev(10000., 600, 30.).iter() {
            particles.push(ParticleConfig::new(
                center + Vec2::NEG_Y * 6.,
                *v,
                thread_rng().gen_range(20..28),
                Duration::from_secs_f32(thread_rng().gen_range(4.8..10.)),
                *colors.iter().choose(&mut thread_rng()).unwrap(),
            ));
        }
        let mut config = FireworkConfig::default()
            .with_gradient_scale(explosion_gradient_1)
            .with_ar_scale(0.09)
            .with_gravity_scale(0.5);
        config.set_enable_gradient(enable_gradient);
        Firework {
            init_time: SystemTime::now(),
            spawn_after,
            center,
            particles,
            config,
            ..Default::default()
        }
    };
    res.push(f2);

    for (idx, p) in gen_points_circle(27, 10).iter().enumerate() {
        let colors = [(17, 138, 178), (6, 214, 160), (7, 59, 76), (255, 255, 255)];
        let mut particles = Vec::new();
        for v in gen_points_circle_normal_dev(100., 35, 350. / 9.).iter() {
            particles.push(ParticleConfig::new(
                center + *p,
                *v,
                thread_rng().gen_range(20..30),
                Duration::from_secs_f32(thread_rng().gen_range(3.0..4.0)),
                *colors.iter().choose(&mut thread_rng()).unwrap(),
            ));
        }
        let mut config = FireworkConfig::default()
            .with_gradient_scale(explosion_gradient_3)
            .with_ar_scale(0.28)
            .with_gravity_scale(0.25);
        config.set_enable_gradient(enable_gradient);
        res.push(Firework {
            init_time: SystemTime::now(),
            spawn_after: spawn_after + Duration::from_secs_f32(0.2 * (idx + 4) as f32),
            center,
            particles,
            config,
            ..Default::default()
        });
    }

    res
}
