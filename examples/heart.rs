use std::{
    f32::consts::PI,
    io::{stdout, Result},
    thread::sleep,
    time::{Duration, SystemTime},
};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, terminal,
};
use firework_rs::{
    fireworks::{ExplosionForm, Firework, FireworkConfig, FireworkManager},
    particle::ParticleConfig,
    term::Terminal,
    utils::gen_points_fan,
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
    let mut fm = FireworkManager::default().with_firework(gen_heart_firework(Vec2::new(
        _width as f32 / 4.,
        _height as f32 / 2.,
    )));

    while is_running {
        if event::poll(Duration::ZERO)? {
            match event::read()? {
                event::Event::Key(e) => {
                    if e.code == KeyCode::Esc {
                        is_running = false;
                    }
                }
                event::Event::Resize(_, _) => {
                    fm.reset();
                    term.reinit();
                }
                _ => {}
            };
        }

        let delta_time = SystemTime::now().duration_since(time).unwrap();
        fm.update(time, delta_time);
        time = SystemTime::now();

        term.render(&fm);
        term.print(&mut stdout);

        if delta_time < Duration::from_secs_f32(0.05) {
            let rem = Duration::from_secs_f32(0.05) - delta_time;
            sleep(rem);
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn gen_heart_firework(center: Vec2) -> Firework {
    let colors = vec![
        (233, 232, 237),
        (254, 142, 130),
        (200, 27, 72),
        (86, 18, 31),
    ];
    let mut particles = Vec::new();
    let trail_length = thread_rng().gen_range(100..105);
    let life_time = Duration::from_secs_f32(thread_rng().gen_range(3.0..3.2));
    let init_pos = center - Vec2::NEG_Y * 15.;
    for v in gen_points_fan(300., 45, 0.2 * PI, 0.3 * PI).iter() {
        particles.push(ParticleConfig::new(
            init_pos,
            *v,
            trail_length,
            life_time,
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    for v in gen_points_fan(300., 45, 0.7 * PI, 0.8 * PI).iter() {
        particles.push(ParticleConfig::new(
            init_pos,
            *v,
            trail_length,
            life_time,
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_ar_scale(0.1)
        .with_gravity_scale(0.1)
        .with_gradient_scale(gradient)
        .with_additional_force(move |particle| (center - particle.pos) * 2.);
    config.set_enable_gradient(true);
    Firework {
        init_time: SystemTime::now(),
        spawn_after: Duration::ZERO,
        center,
        particles,
        config,
        form: ExplosionForm::Instant { used: false },
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
