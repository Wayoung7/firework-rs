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
    let mut fm = FireworkManager::default().with_firework(gen_fountain_firework(Vec2::new(
        _width as f32 / 4.,
        _height as f32 / 2. + 13.,
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

fn gen_fountain_firework(center: Vec2) -> Firework {
    let colors = vec![(226, 196, 136), (255, 245, 253), (208, 58, 99)];
    let mut particles = Vec::new();
    for v in gen_points_fan(
        300.,
        45,
        5 as f32 / 12 as f32 * PI,
        7 as f32 / 12 as f32 * PI,
    )
    .iter()
    {
        particles.push(ParticleConfig::new(
            center,
            *v,
            thread_rng().gen_range(28..38),
            Duration::from_secs_f32(thread_rng().gen_range(2.5..3.8)),
            *colors.iter().choose(&mut thread_rng()).unwrap(),
        ));
    }
    let mut config = FireworkConfig::default()
        .with_ar_scale(0.15)
        .with_gravity_scale(0.5)
        .with_gradient_scale(gradient);
    config.set_enable_gradient(true);
    Firework {
        init_time: SystemTime::now(),
        spawn_after: Duration::ZERO,
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
}

fn gradient(x: f32) -> f32 {
    if x < 0.8125 {
        -0.4 * x + 1.1
    } else {
        -2. * x + 2.4
    }
}
