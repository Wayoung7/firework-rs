mod demo;
mod firework;
mod particle;
mod term;
mod utils;

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
use demo::{demo_firework_1, demo_firework_2, demo_firework_comb_1};
use firework::FireworkManager;
use glam::Vec2;
use term::Terminal;

fn main() -> Result<()> {
    let mut stdout = stdout();
    let (width, height) = terminal::size()?;
    let mut is_running = true;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut time = SystemTime::now();
    let mut term = Terminal::default();
    let mut fm = FireworkManager::default()
        .add_fireworks(demo_firework_comb_1(
            Vec2::new(30., 66.),
            Duration::from_secs_f32(0.2),
        ))
        .enable_loop();
    // .add_firework(demo_firework_1(
    //     Vec2::new(20., 15.),
    //     Duration::from_secs_f32(1.),
    // ))
    // .add_firework(demo_firework_2(
    //     Vec2::new(40., 25.),
    //     Duration::from_secs_f32(1.3),
    // ))
    // .add_firework(demo_firework_1(
    //     Vec2::new(15., 45.),
    //     Duration::from_secs_f32(2.),
    // ));

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
