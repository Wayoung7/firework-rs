//! With the `firework` binary, you can run some pre-designed fireworks with command line arguments

mod args;

use std::{
    io::{stdout, Result},
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
use firework_rs::demo::{
    demo_firework_2, demo_firework_comb_0, demo_firework_comb_1, demo_firework_comb_2,
    demo_firework_comb_3,
};
use firework_rs::fireworks::FireworkManager;
use firework_rs::term::Terminal;
use glam::Vec2;

fn main() -> Result<()> {
    let mut is_running = true;
    let cli = Cli::parse();
    let (_width, _height) = terminal::size()?;
    let mut fm = match cli.demo {
        0 => FireworkManager::default().add_fireworks(demo_firework_comb_0(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        1 => FireworkManager::default().add_fireworks(demo_firework_comb_2(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        2 => FireworkManager::default().add_fireworks(demo_firework_comb_3(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        3 => FireworkManager::default().add_fireworks(demo_firework_comb_1(
            Vec2::new(_width as f32 / 4., 66.),
            Duration::from_secs_f32(0.2),
            cli.gradient,
        )),
        4 => FireworkManager::default().add_firework(demo_firework_2(
            Vec2::new(_width as f32 / 4., _height as f32 / 2.),
            Duration::from_secs_f32(0.7),
            cli.gradient,
        )),
        _ => {
            println!("Demo number error\n");
            is_running = false;
            FireworkManager::default()
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
