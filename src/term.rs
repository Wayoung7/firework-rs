use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, queue, style, terminal};
use glam::Vec2;

use crate::{
    fireworks::{FireworkManager, FireworkState},
    particle::{
        construct_line, get_char_alive, get_char_declining, get_char_dying, shift_gradient,
        LifeState,
    },
};

/// Wrap a character with color
#[derive(Debug, Clone, Copy)]
pub struct Char {
    pub text: char,
    pub color: style::Color,
}

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
    /// Clear the terminal screen by setting all the characters in terminal to space
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
                .unwrap()
            });
        });
        w.flush().unwrap();
    }

    /// Write the rendering data of all `Fireworks` and `Particles` to `Terminal`
    pub fn render(&mut self, fm: &FireworkManager) {
        // self.clear_screen();
        // for firework in fireworks.fireworks.iter() {
        //     if firework.state == FireworkState::Alive {
        //         for particle in firework.current_particles.iter() {
        //             particle
        //                 .draw(&firework.config)
        //                 .iter()
        //                 .for_each(|((x, y), (char, color))| {
        //                     if self.inside((*x, *y)) {
        //                         self.screen[*y as usize][*x as usize] = Char::new(
        //                             *char,
        //                             style::Color::Rgb {
        //                                 r: color.0,
        //                                 g: color.1,
        //                                 b: color.2,
        //                             },
        //                         );
        //                     }
        //                 });
        //         }
        //     }
        // }
        self.clear_screen();
        for firework in fm.fireworks.iter() {
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
        if x < self.size.0 as isize && y < self.size.1 as isize && x >= 0 && y >= 0 {
            true
        } else {
            false
        }
    }
}
