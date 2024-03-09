use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Result, Stdout, Write},
};

use crossterm::{cursor::MoveTo, execute, queue, style, terminal};
use glam::Vec2;

use crate::{
    firework::{FireworkManager, FireworkState},
    particle::Particle,
};

#[derive(Debug, Clone, Copy)]
pub struct Char {
    pub text: char,
    pub color: style::Color,
}

impl Char {
    fn new(text: char, color: style::Color) -> Self {
        Self { text, color }
    }
}

pub struct Terminal {
    pub size: (u16, u16),
    pub screen: Vec<Vec<Char>>,
}

impl Default for Terminal {
    fn default() -> Self {
        let size = terminal::size().unwrap();
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
    pub fn clear_screen(&mut self) {
        let size = terminal::size().unwrap();
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

    pub fn print(&self, w: &mut Stdout) {
        self.screen.iter().enumerate().for_each(|(y, line)| {
            line.iter().enumerate().for_each(|(x, c)| {
                queue!(
                    w,
                    MoveTo(x as u16, y as u16),
                    style::SetForegroundColor(c.color),
                    style::Print(c.text)
                )
                .unwrap()
            });
        });
        w.flush().unwrap();
    }

    pub fn render(&mut self, fireworks: &FireworkManager) {
        self.clear_screen();
        for firework in fireworks.fireworks.iter() {
            if firework.state == FireworkState::Alive {
                for particle in firework.particles.iter() {
                    particle
                        .draw(&firework.config)
                        .iter()
                        .for_each(|((x, y), (char, color))| {
                            if self.inside((*x, *y)) {
                                self.screen[*y as usize][*x as usize] = Char::new(
                                    *char,
                                    style::Color::Rgb {
                                        r: color.0,
                                        g: color.1,
                                        b: color.2,
                                    },
                                );
                            }
                        });
                }
            }
        }
    }

    fn inside(&self, (x, y): (isize, isize)) -> bool {
        if x < self.size.0 as isize && y < self.size.1 as isize && x >= 0 && y >= 0 {
            true
        } else {
            false
        }
    }
}
