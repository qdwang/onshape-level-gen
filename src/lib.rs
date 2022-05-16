//! This crate contains essential utilities to generate an OnShape yml level file.
//! 
//! ## Example
//! ```
//! use onshape_level_gen::util::*;
//! 
//! // this difficulty is a FFT result iterating factor.
//! let init_difficulty_limit = 1.2f32;
//! 
//! let (samples, output_params) = get_data_from_ogg(path).unwrap();
//! let notes = get_notes_from_samples(samples, init_difficulty_limit).unwrap();
//! let walls = get_walls_from_notes(&notes);
//! let yml_content = gen_yml(&output_params, walls);
//! 
//! std::fs::write("~/level.yml", yml_content).unwrap();
//! ```
//! 
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::{string::ToString, fmt::Display};

pub mod util;
pub mod wall;

/// A collection of raw f32 data with sample rate from sound file.
pub struct Samples {
    rate: u32,
    data: Vec<f32>,
}

/// The FFT result of a fixed segmenet of samples.
pub struct Note {
    pub freq: f32,
    pub val: f32,
    pub time: f32,
}

/// A most common type(left, center, right) in the OnShape game.
#[derive(Debug, Copy, Clone)]
pub enum LCR {
    L,
    C,
    R,
}

/// Represent the 5 dodge types in the OnShape game.
#[derive(Debug, Copy, Clone)]
pub enum DodgeType {
    TopLeft,
    Left,
    Top,
    Right,
    TopRight,
}

/// All 4 possible shapes in the OnShape game.
#[derive(Debug, Clone)]
pub enum WallType {
    Shape {
        lean: LCR,
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

/// Represents the time and the type of a wall.
#[derive(Debug, Clone)]
pub struct Wall(f32, WallType);

/// 4 Difficulties used in OnShape level script.
pub enum Difficulty {
    Beginner, Easy, Medium, Hard
}
/// The parameters needed to generate the yml file.
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
        match rng.gen_range(0..=3) {
            0 => LCR::L,
            1 => LCR::C,
            2 => LCR::C,
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
