use std::{
    f32::consts::PI,
    fs,
    io::{stdout, Result, Write},
    thread::sleep,
    time::{Duration, Instant, SystemTime},
};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, terminal,
};
use firework::{
    fireworks::{ExplosionForm, Firework, FireworkConfig, FireworkManager},
    particle::{Particle, ParticleConfig},
    term::Terminal,
    utils::{gen_points_circle, gen_points_fan},
};
use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng, Rng};

fn main() -> Result<()> {
    let mut stdout = stdout();
    let (_width, _height) = terminal::size()?;
    let mut is_running = true;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut time = SystemTime::now();
    let mut term = Terminal::default();
    let mut fm = FireworkManager::default().add_firework(gen_vortex_firework(Vec2::new(
        _width as f32 / 4.,
        _height as f32 / 2.,
    )));

    while is_running {
        let delta_time = SystemTime::now().duration_since(time).unwrap();
        fm.update(time, delta_time);
        time = SystemTime::now();

        term.render(&fm);
        term.print(&mut stdout);

        if event::poll(Duration::ZERO)? {
            match event::read()? {
                event::Event::Key(e) => {
                    if e.code == KeyCode::Esc {
                        is_running = false;
                    }
                }
                _ => {}
            };
        }

        if delta_time < Duration::from_secs_f32(0.05) {
            let rem = Duration::from_secs_f32(0.05) - delta_time;
            sleep(rem);
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn gen_vortex_firework(center: Vec2) -> Firework {
    let colors = vec![
        (233, 232, 237),
        (254, 142, 130),
        (200, 27, 72),
        (86, 18, 31),
    ];
    let mut particles = Vec::new();
    for p in gen_points_circle(30, 45).iter() {
        particles.push(ParticleConfig::new(
            center + *p,
            Vec2::new((*p).y, -(*p).x).normalize() * 15.,
            thread_rng().gen_range(28..40),
            Duration::from_secs_f32(thread_rng().gen_range(4.5..7.0)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_ar_scale(0.05)
        .with_gravity_scale(0.)
        .with_gradient_scale(gradient)
        .with_additional_force(move |particle| {
            (center - particle.pos).normalize() * (1. / center.distance(particle.pos)) * 150.
        });
    config.set_enable_gradient(true);
    Firework {
        init_time: SystemTime::now(),
        spawn_after: Duration::ZERO,
        center,
        particles,
        config,
        form: ExplosionForm::Sustained {
            lasts: Duration::from_secs(10),
            time_interval: Duration::from_secs_f32(0.01),
            timer: Duration::ZERO,
        },
        ..Default::default()
    }
}

fn gradient(x: f32) -> f32 {
    if x < 0.8125 {
        -0.4 * x + 1.1
    } else {
        -2. * x + 2.2
    }
}
