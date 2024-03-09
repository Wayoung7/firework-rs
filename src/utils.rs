use glam::Vec2;
use rand::Rng;
use rand_distr::Distribution;

pub fn round(input: Vec2) -> (isize, isize) {
    (input.x.round() as isize, input.y.round() as isize)
}

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

pub fn gen_points_circle_normal(radius: f32, n: usize) -> Vec<Vec2> {
    let mut rng = rand::thread_rng();
    let normal = rand_distr::Normal::new(0., radius / 9.).unwrap();
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

pub fn distance_squared(a: Vec2, b: Vec2) -> f32 {
    (b.x - a.x).powi(2) + (b.y - a.y).powi(2)
}

pub fn explosion_gradient_1(x: f32) -> f32 {
    if x < 0.087 {
        150. * x.powi(2)
    } else {
        -0.8 * x + 1.2
    }
}

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

pub fn linear_gradient_1(x: f32) -> f32 {
    -0.7 * x + 1.
}
