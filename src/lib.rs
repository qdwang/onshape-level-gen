//! Library used in onshape_level_gen
//! 
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{string::ToString, fmt::Display};

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

#[derive(Debug, Copy, Clone)]
pub enum LCR {
    L,
    C,
    R,
}
#[derive(Debug, Copy, Clone)]
pub enum DodgeType {
    TopLeft,
    Left,
    Top,
    Right,
    TopRight,
}
#[derive(Debug, Clone)]
pub enum WallType {
    Shape {
        position: LCR,
        standing: bool,
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
        y: i8  // from 13 to 0
    }
}

#[derive(Debug, Clone)]
pub struct Wall {
    time: f32,
    t: WallType,
}

pub enum Difficulty {
    Beginner, Easy, Medium, Hard
}
pub struct OutputParams {
    title: String,
    pub output_file: String,
    speed: u8,
    audio_total_time: f32,
    difficulty: Difficulty,
}



impl LCR {
    pub fn to_str(&self, center_char: Option<&'static str>) -> &'static str {
        match &self {
            LCR::L => "L",
            LCR::C => center_char.unwrap_or("C"),
            LCR::R => "R",
        }
    }
}

impl DodgeType {
    pub fn to_str(&self) -> &'static str {
        match &self {
            DodgeType::TopLeft => "LI",
            DodgeType::Left => "L",
            DodgeType::Top => "U",
            DodgeType::Right => "R",
            DodgeType::TopRight => "RI"
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match &self {
            Difficulty::Beginner => "beginner",
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        };
        write!(f, "{v}")
    }
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
