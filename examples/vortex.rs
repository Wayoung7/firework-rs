use std::{
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
    utils::gen_points_circle,
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
    let mut fm = FireworkManager::default().with_firework(gen_vortex_firework(Vec2::new(
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

fn gen_vortex_firework(center: Vec2) -> Firework {
    let colors = vec![
        (6, 55, 63),
        (24, 90, 96),
        (47, 123, 119),
        (92, 174, 166),
        (200, 255, 255),
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
