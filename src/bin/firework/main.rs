//! With the `firework` binary, you can run some pre-designed fireworks with command line arguments

mod args;
mod gen;

use std::{
    io::{stdout, Error, Result},
    thread::sleep,
    time::{Duration, SystemTime},
};

use args::Cli;
use clap::Parser;
use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, terminal,
};
use firework_rs::fireworks::FireworkManager;
use firework_rs::term::Terminal;
use firework_rs::{
    demo::{
        demo_firework_2, demo_firework_comb_0, demo_firework_comb_1, demo_firework_comb_2,
        demo_firework_comb_3,
    },
    fireworks::FireworkInstallForm,
};
use gen::dyn_gen;
use glam::Vec2;

fn main() -> Result<()> {
    let mut fps: u8 = 20;
    let mut is_running = true;
    let cli = Cli::parse();
    if let Some(f) = cli.fps {
        if !(5..=30).contains(&f) {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Invalid fps value! Valid fps range: 5~30",
            ));
        } else {
            fps = f;
        }
    }
    let (mut _width, mut _height) = terminal::size()?;
    let mut fm = match cli.demo {
        Some(0) => FireworkManager::default().with_fireworks(demo_firework_comb_0(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        Some(1) => FireworkManager::default().with_fireworks(demo_firework_comb_2(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        Some(2) => FireworkManager::default().with_fireworks(demo_firework_comb_3(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        Some(3) => FireworkManager::default().with_fireworks(demo_firework_comb_1(
            Vec2::new(_width as f32 / 4., 66.),
            Duration::from_secs_f32(0.2),
            cli.gradient,
        )),
        Some(4) => FireworkManager::default().with_firework(demo_firework_2(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        None => FireworkManager::default().enable_dyn_install(),
        _ => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Invalid demo number! Demo number should be: 0~4",
            ));
        }
    };
    fm.set_enable_loop(cli.looping);

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut time = SystemTime::now();
    let mut term = Terminal::default();

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

        (_width, _height) = terminal::size()?;
        let delta_time = SystemTime::now().duration_since(time).unwrap();
        if fm.install_form == FireworkInstallForm::DynamicInstall {
            dyn_gen(&mut fm, _width, _height, cli.gradient);
        }
        fm.update(time, delta_time);
        time = SystemTime::now();
        term.render(&fm);
        term.print(&mut stdout);

        if delta_time < Duration::from_secs_f32(1. / fps as f32) {
            let rem = Duration::from_secs_f32(1. / fps as f32) - delta_time;
            sleep(rem);
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
