//! `term` module provides functions of rendering in terminal

use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, queue, style, terminal};
use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng};

use crate::{
    fireworks::{FireworkManager, FireworkState},
    particle::LifeState,
    utils::distance_squared,
};

/// Wrap a character with color
#[derive(Debug, Clone, Copy)]
pub struct Char {
    pub text: char,
    pub color: style::Color,
}

#[allow(unused)]
impl Char {
    /// Create a new `Char`
    fn new(text: char, color: style::Color) -> Self {
        Self { text, color }
    }
}

/// Struct that represents a terminal
pub struct Terminal {
    pub size: (u16, u16),
    pub screen: Vec<Vec<Char>>,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = terminal::size().expect("Fail to get terminal size.");
        let mut screen = Vec::new();
        (0..size.1).for_each(|_| {
            let mut line = Vec::new();
            (0..size.0).for_each(|_| {
                line.push(Char {
                    text: ' ',
                    color: style::Color::White,
                })
            });
            screen.push(line);
        });
        Self { size, screen }
    }
}

impl Terminal {
    /// Reload terminal to adapt new window size
    pub fn reinit(&mut self) {
        let size = terminal::size().expect("Fail to get terminal size.");
        let mut screen = Vec::new();
        (0..size.1).for_each(|_| {
            let mut line = Vec::new();
            (0..size.0).for_each(|_| {
                line.push(Char {
                    text: ' ',
                    color: style::Color::White,
                })
            });
            screen.push(line);
        });
        self.screen = screen;
        self.size = size;
    }

    /// Clear the terminal screen by setting all the characters in terminal to space
    pub fn clear_screen(&mut self) {
        let size = terminal::size().expect("Fail to get terminal size.");
        let mut s = Vec::new();
        (0..size.1).for_each(|_| {
            let mut line = Vec::new();
            (0..size.0).for_each(|_| {
                line.push(Char {
                    text: ' ',
                    color: style::Color::White,
                })
            });
            s.push(line);
        });
        self.screen = s;
    }

    /// Print the data out to terminal
    pub fn print(&self, w: &mut Stdout) {
        self.screen.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| {
                queue!(
                    w,
                    MoveTo(x as u16, y as u16),
                    style::SetForegroundColor(c.color),
                    style::Print(c.text)
                )
                .expect("Std io error.")
            });
        });
        w.flush().expect("Std io error.");
    }

    /// Write the rendering data of all `Fireworks` and `Particles` to `Terminal`
    pub fn render(&mut self, fm: &FireworkManager) {
        self.clear_screen();
        for firework in fm.fireworks.iter().rev() {
            if firework.state == FireworkState::Alive {
                for particle in firework.current_particles.iter().rev() {
                    let grad = if firework.config.enable_gradient {
                        Some((firework.config.gradient_scale)(
                            particle.time_elapsed.as_secs_f32()
                                / particle.config.life_time.as_secs_f32(),
                        ))
                    } else {
                        None
                    };
                    particle
                        .trail
                        .iter()
                        .map(|p| Vec2::new(p.x * 2., p.y))
                        .rev()
                        .collect::<Vec<_>>()
                        .windows(2)
                        .enumerate()
                        .for_each(|(idx, v)| {
                            let density = (particle.config.trail_length - idx - 1) as f32
                                / particle.config.trail_length as f32;
                            construct_line(v[0], v[1]).iter().for_each(|p| {
                                if self.inside(*p)
                                    && self.screen[p.1 as usize][p.0 as usize].text == ' '
                                {
                                    if let Some(c) = match particle.life_state {
                                        LifeState::Alive => Some(get_char_alive(density)),
                                        LifeState::Declining => Some(get_char_declining(density)),
                                        LifeState::Dying => Some(get_char_dying(density)),
                                        LifeState::Dead => None,
                                    } {
                                        self.screen[p.1 as usize][p.0 as usize] = Char {
                                            text: c,
                                            color: {
                                                let color_u8 = if let Some(g) = grad {
                                                    shift_gradient(particle.config.color, g)
                                                } else {
                                                    particle.config.color
                                                };
                                                style::Color::Rgb {
                                                    r: color_u8.0,
                                                    g: color_u8.1,
                                                    b: color_u8.2,
                                                }
                                            },
                                        }
                                    }
                                }
                            });
                        });
                }
            }
        }
    }

    fn inside(&self, (x, y): (isize, isize)) -> bool {
        x < self.size.0 as isize && y < self.size.1 as isize && x >= 0 && y >= 0
    }
}

fn construct_line(a: Vec2, b: Vec2) -> Vec<(isize, isize)> {
    const STEP: f32 = 0.2;
    let (x0, y0) = (a.x, a.y);
    let (x1, y1) = (b.x, b.y);
    let mut path = Vec::new();
    let mut x = x0;
    let mut y = y0;
    let slope = (y1 - y0) / (x1 - x0);
    let dx = if x0 == x1 {
        0.
    } else if x1 > x0 {
        1.
    } else {
        -1.
    };
    let dy = if y0 == y1 {
        0.
    } else if y1 > y0 {
        1.
    } else {
        -1.
    };
    let mut ds = distance_squared(a, b) + f32::EPSILON;
    path.push((x0.round() as isize, y0.round() as isize));
    if (x1 - x0).abs() >= (y1 - y0).abs() {
        while distance_squared(Vec2::new(x, y), b) <= ds {
            if *path.last().unwrap() != (x.round() as isize, y.round() as isize) {
                path.push((x.round() as isize, y.round() as isize));
                ds = distance_squared(Vec2::new(x, y), b);
            }
            x += dx * STEP;
            y += dy * (STEP * slope).abs();
        }
    } else {
        while distance_squared(Vec2::new(x, y), b) <= ds {
            if *path.last().unwrap() != (x.round() as isize, y.round() as isize) {
                path.push((x.round() as isize, y.round() as isize));
                ds = distance_squared(Vec2::new(x, y), b);
            }
            y += dy * STEP;
            x += dx * (STEP / slope).abs();
        }
    }
    path
}

fn shift_gradient(color: (u8, u8, u8), scale: f32) -> (u8, u8, u8) {
    (
        (color.0 as f32 * scale) as u8,
        (color.1 as f32 * scale) as u8,
        (color.2 as f32 * scale) as u8,
    )
}

fn get_char_alive(density: f32) -> char {
    let palette = if density < 0.3 {
        "`'. "
    } else if density < 0.5 {
        "/\\|()1{}[]?"
    } else if density < 0.7 {
        "oahkbdpqwmZO0QLCJUYXzcvunxrjft*"
    } else {
        "$@B%8&WM#"
    };
    palette
        .chars()
        .choose(&mut thread_rng())
        .expect("Fail to choose character.")
}

fn get_char_declining(density: f32) -> char {
    let palette = if density < 0.2 {
        "` '. "
    } else if density < 0.6 {
        "-_ +~<> i!lI;:,\"^"
    } else if density < 0.85 {
        "/\\| ()1{}[ ]?"
    } else {
        "xrjft*"
    };
    palette
        .chars()
        .choose(&mut thread_rng())
        .expect("Fail to choose character.")
}

fn get_char_dying(density: f32) -> char {
    let palette = if density < 0.6 {
        ".  ,`.    ^,' . "
    } else {
        " /\\| ( )  1{} [  ]?i !l I;: ,\"^ "
    };
    palette
        .chars()
        .choose(&mut thread_rng())
        .expect("Fail to choose character.")
}
