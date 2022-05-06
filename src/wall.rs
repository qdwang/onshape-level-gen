//! This file contains implementations for Wall module.
//!
use super::*;
use rand::rngs::ThreadRng;

impl Wall {
    pub fn new(note: &Note, time2next: f32, rng: &mut ThreadRng) -> Self {
        let wall_type = match rng.gen_range(0..=3) {
            0 => WallType::Shape {
                hands_over: rand::random(),
                position: rand::random(),
                lean: rand::random(),
                standing: rand::random(),
                leg_lunge: rand::random(),
            },
            1 => WallType::Hit {
                position: rand::random(),
                standing: rand::random(),
                hands: rand::random(),
            },
            2 => WallType::Dodge {
                t: rand::random(),
                duration: (time2next * 100.) as u16 - rng.gen_range(100..=200),
            },
            _ => WallType::Coin {
                x: rng.gen_range(-10..=10),
                y: rng.gen_range(0..=13),
            },
        };

        Wall {
            time: note.time,
            t: wall_type,
        }
    }

    pub fn to_code(&self) -> String {
        match self.t {
            WallType::Shape {
                hands_over,
                position,
                lean,
                standing,
                leg_lunge,
            } => {
                let result = match (hands_over, position, lean, standing, leg_lunge) {
                    _ => "WP.C22CDC",
                };
                result.to_owned()
            }
            _ => "".to_owned(),
        }
    }
}
