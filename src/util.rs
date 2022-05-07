//! This file contains general functions.
//!
use std::path::Path;

use super::*;
use creak::{Decoder, DecoderError};
use spectrum_analyzer::error::*;
use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

/// Get the one channel `Vec<f32>` data and output parameters from an ogg file.
///
/// The samples data from 2 channels will be combined into one channel by averaging the values.
pub fn get_data_from_ogg(path: &str) -> Result<(Samples, OutputParams), DecoderError> {
    println!("Reading file: {}", path);
    let decoder = Decoder::open(path)?;
    let info = decoder.info();

    let data: Vec<f32> = decoder
        .into_samples()?
        .collect::<Result<Vec<f32>, DecoderError>>()?;
    let data: Vec<f32> = data.chunks_exact(2).map(|x| (x[0] + x[1]) / 2.0).collect();

    let path = Path::new(path);
    let title = path
        .file_stem()
        .and_then(|x| x.to_str())
        .unwrap_or(&"file_name")
        .to_owned();
    let output_file = path
        .with_extension("yml")
        .to_str()
        .unwrap_or(&"output_file")
        .to_owned();
    let speed = 30;
    let audio_total_time = data.len() as f32 / info.sample_rate() as f32;
    let difficulty = Difficulty::Easy;

    Ok((
        Samples {
            rate: info.sample_rate(),
            data,
        },
        OutputParams {
            title,
            output_file,
            speed,
            audio_total_time,
            difficulty,
        },
    ))
}

/// Get a series of `Note` from a `Samples` object.
///
/// The core shapes generating algorithm is inside this function.
pub fn get_notes_from_samples(samples: Samples) -> Result<Vec<Note>, SpectrumAnalyzerError> {
    println!("Getting notes");

    const FFT_SIZE: usize = 4096;
    const MIN_HZ: f32 = 50.;
    const MAX_HZ: f32 = 6000.;

    let mut difficulty = 0f32;
    let mut result = vec![];

    for (block_index, block) in samples.data.chunks_exact(FFT_SIZE).enumerate() {
        let time = (block_index * FFT_SIZE) as f32 / samples.rate as f32;

        let spectrum_hann_window = samples_fft_to_spectrum(
            &hann_window(block),
            samples.rate,
            FrequencyLimit::Range(MIN_HZ, MAX_HZ),
            Some(&divide_by_N),
        )?;

        // core part lists here
        let (freq, val) = spectrum_hann_window.data().iter().fold(
            (f32::MIN, f32::MIN),
            |(acc_freq, acc_val), (freq, val)| {
                if val.val() > acc_val {
                    (freq.val(), val.val())
                } else {
                    (acc_freq, acc_val)
                }
            },
        );

        difficulty = (difficulty + val) / 2.0f32;

        if val > 0.001 && val > difficulty * 1.2 {
            result.push(Note { freq, val, time });
        }
    }

    Ok(result)
}

pub fn get_walls_from_notes(notes: &[Note]) -> Vec<Wall> {
    let mut rng = rand::thread_rng();
    let mut acc_coin_time = 0f32;
    let mut prev_wall = None;

    notes
        .windows(2)
        .map(|notes| {
            let time2next = notes[1].time - notes[0].time;
            Wall::new(&notes[0], &mut rng, time2next, &mut acc_coin_time, &mut prev_wall)
        })
        .collect()
}

pub fn gen_yml(
    OutputParams {
        title,
        output_file: _,
        speed,
        audio_total_time,
        difficulty,
    }: &OutputParams,
    walls: Vec<Wall>,
) -> String {
    println!("Generating yml content");

    let mut result = format!(
        "
%YAML 1.1
---

title: {title}
clip: {title}
speed: {speed}
audioTime: {audio_total_time:.3}
scenario: 0

video: 
vOffset: 0

forceDebug: False
offset: 0

author: ohshape level gen
difficulty: {difficulty}
preview: 0

grid: False
gridBpm: 0
gridOffset: 0


levels:
  - level: 0
    sequence:

"
    );

    for wall in walls {
        let time = format!("{:.2}", wall.time);
        let code = wall.to_code();

        result.push_str(
            format!("      - second: {time}\n        obj: {code}\n        track: 0\n").as_str(),
        );
    }

    result.push_str("...");
    result
}
