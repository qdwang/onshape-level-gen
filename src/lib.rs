//! Library used in onshape_level_gen
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub mod util;
pub mod wall;

pub struct Samples {
    rate: u32,
    data: Vec<f32>,
}

pub struct Note {
    pub freq: f32,
    pub val: f32,
    pub time: f32,
}

#[derive(Copy, Clone)]
pub enum LCR {
    L,
    C,
    R,
}
#[derive(Copy, Clone)]
pub enum DodgeType {
    TopLeft,
    Left,
    Top,
    Right,
    TopRight,
}
pub enum WallType {
    Shape {
        hands_over: bool,
        position: LCR,
        lean: LCR,
        standing: bool,
        leg_lunge: LCR,
    },
    Hit {
        position: LCR,
        standing: bool,
        hands: LCR,
    },
    Dodge {
        t: DodgeType,
        duration: u16,
    },
    Coin {
        x: i8, // from -10 to 10
        y: u8  // from 13 to 0
    }
}

pub struct Wall {
    time: f32,
    t: WallType,
}


impl Distribution<LCR> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LCR {
        match rng.gen_range(0..=2) {
            0 => LCR::L,
            1 => LCR::C,
            _ => LCR::R,
        }
    }
}

impl Distribution<DodgeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DodgeType {
        match rng.gen_range(0..=4) {
            0 => DodgeType::TopLeft,
            1 => DodgeType::Left,
            2 => DodgeType::Top,
            3 => DodgeType::Right,
            _ => DodgeType::TopRight,
        }
    }
}
