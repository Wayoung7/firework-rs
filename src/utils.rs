//! `utils` module provides some useful helper functions of random generation and gradient scale

use std::f32::consts::PI;

use glam::Vec2;
use rand::Rng;
use rand_distr::Distribution;

/// Round a `Vec2` from `(f32, f32)` to `(isize, isize)`
pub fn round(input: Vec2) -> (isize, isize) {
    (input.x.round() as isize, input.y.round() as isize)
}

/// Generate random `Vec2` within a circle range
pub fn gen_points_circle(radius: isize, n: usize) -> Vec<Vec2> {
    let mut res = Vec::new();
    while res.len() < n {
        let x = rand::thread_rng().gen_range(-radius..=radius);
        let y = rand::thread_rng().gen_range(-radius..=radius);
        if x.pow(2) + y.pow(2) <= radius.pow(2) {
            res.push(Vec2::new(x as f32, y as f32));
        }
    }
    res
}

/// Generate random `Vec2` within a circle range with normal distribution
///
/// Points closer to the center will be denser
pub fn gen_points_circle_normal(radius: f32, n: usize) -> Vec<Vec2> {
    let mut rng = rand::thread_rng();
    let normal =
        rand_distr::Normal::new(0., radius / 9.).expect("Unable to generate normal distribution.");
    let mut res = Vec::new();
    while res.len() < n {
        let x = normal.sample(&mut rng);
        if x < -radius || x > radius {
            continue;
        }
        let y = normal.sample(&mut rng);
        if x < -radius || y > radius {
            continue;
        }
        if x.powi(2) + y.powi(2) <= radius.powi(2) {
            res.push(Vec2::new(x, y));
        }
    }
    res
}

/// Generate random `Vec2` within a circle range with normal distribution
///
/// Points closer to the center will be denser
/// You can specify standard deviation yourself
pub fn gen_points_circle_normal_dev(radius: f32, n: usize, std_dev: f32) -> Vec<Vec2> {
    let mut rng = rand::thread_rng();
    let normal =
        rand_distr::Normal::new(0., std_dev).expect("Unable to generate normal distribution.");
    let mut res = Vec::new();
    while res.len() < n {
        let x = normal.sample(&mut rng);
        if x < -radius || x > radius {
            continue;
        }
        let y = normal.sample(&mut rng);
        if x < -radius || y > radius {
            continue;
        }
        if x.powi(2) + y.powi(2) <= radius.powi(2) {
            res.push(Vec2::new(x, y));
        }
    }
    res
}

/// Generate random `Vec2` within a fan-shape range
pub fn gen_points_fan(radius: f32, n: usize, st_angle: f32, ed_angle: f32) -> Vec<Vec2> {
    let mut res = Vec::new();
    while res.len() < n {
        let x = rand::thread_rng().gen_range(-radius..=radius);
        let y = rand::thread_rng().gen_range(-radius..=radius);
        let t = y.atan2(x);
        if t <= ed_angle && t >= st_angle && x.powi(2) + y.powi(2) <= radius.powi(2) {
            res.push(Vec2::new(x, -y));
        }
    }
    res
}

/// Generate random `Vec2` on an arc
pub fn gen_points_arc(radius: f32, n: usize, st_angle: f32, ed_angle: f32) -> Vec<Vec2> {
    let mut res = Vec::new();
    while res.len() < n {
        let a = rand::thread_rng().gen_range(st_angle..=ed_angle);
        res.push(Vec2::new(radius * a.cos(), -radius * a.sin()));
    }
    res
}

/// Generate random `Vec2` on a circle
pub fn gen_points_on_circle(radius: f32, n: usize) -> Vec<Vec2> {
    let mut res = Vec::new();
    while res.len() < n {
        let a = rand::thread_rng().gen_range(0.0..PI);
        res.push(Vec2::new(radius * a.cos(), -radius * a.sin()));
    }
    res
}

/// Return squared distance between to points
pub fn distance_squared(a: Vec2, b: Vec2) -> f32 {
    (b.x - a.x).powi(2) + (b.y - a.y).powi(2)
}

/// A sample function defining the gradient of the `Particle`
///
/// The visual effect is similar to an explosion
pub fn explosion_gradient_1(x: f32) -> f32 {
    if x < 0.087 {
        150. * x.powi(2)
    } else {
        -0.8 * x + 1.2
    }
}

/// A sample function defining the gradient of the `Particle`
///
/// The visual effect is similar to an explosion
pub fn explosion_gradient_2(x: f32) -> f32 {
    if x < 0.067 {
        5. * x + 0.1
    } else if x < 0.2 {
        2. * x + 0.3
    } else if x < 0.5 {
        x + 0.5
    } else if x < 0.684 {
        0.5 * x + 0.75
    } else {
        -7. * (x - 0.65).powi(2) + 1.1
    }
}

/// A sample function defining the gradient of the `Particle`
///
/// The visual effect is similar to an explosion, darkar than `explosion_gradient_1`
pub fn explosion_gradient_3(x: f32) -> f32 {
    if x < 0.087 {
        150. * x.powi(2) * 0.6
    } else {
        (-0.8 * x + 1.2) * 0.6
    }
}

/// A sample function defining the gradient of the `Particle`
///
/// Linear gradient
pub fn linear_gradient_1(x: f32) -> f32 {
    -0.7 * x + 1.
}
