use std::time::Duration;

use firework_rs::{demo::demo_firework_0, fireworks::FireworkManager};
use glam::Vec2;
use rand::{seq::IteratorRandom, thread_rng, Rng};

pub fn dyn_gen(fm: &mut FireworkManager, width: u16, height: u16, enable_gradient: bool) {
    let colors = [
        vec![
            (255, 102, 75),
            (144, 56, 67),
            (255, 225, 124),
            (206, 32, 41),
        ],
        vec![
            (235, 39, 155),
            (250, 216, 68),
            (242, 52, 72),
            (63, 52, 200),
            (255, 139, 57),
        ],
        vec![
            (152, 186, 227),
            (89, 129, 177),
            (54, 84, 117),
            (240, 244, 254),
        ],
        vec![
            (34, 87, 122),
            (56, 163, 165),
            (87, 204, 153),
            (128, 237, 153),
            (199, 249, 204),
        ],
        vec![
            (205, 180, 219),
            (255, 200, 221),
            (255, 175, 204),
            (189, 224, 254),
            (162, 210, 255),
        ],
        vec![
            (79, 0, 11),
            (114, 0, 38),
            (206, 66, 87),
            (255, 127, 81),
            (255, 155, 84),
        ],
        vec![(0, 29, 61), (0, 53, 102), (255, 195, 0), (255, 214, 10)],
    ];
    if fm.fireworks.len() < (width as usize * height as usize) / 1300 + 3 {
        let x: isize = thread_rng().gen_range(-3..(width as isize + 3));
        let y: isize = thread_rng().gen_range(-1..(height as isize + 1));
        fm.add_firework(demo_firework_0(
            Vec2::new(x as f32, y as f32),
            Duration::from_secs_f32(thread_rng().gen_range(0.0..2.0)),
            enable_gradient,
            colors.iter().choose(&mut thread_rng()).unwrap().to_owned(),
        ));
    }
}
