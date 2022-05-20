//! Contains general and utility functions.
//!
use std::path::Path;

use super::*;
use creak::{Decoder, DecoderError};
use spectrum_analyzer::error::*;
use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

/// Gets the one channel `Vec<f32>` data and output parameters from an ogg file.
///
/// The samples data from 2 channels will be combined into one channel by averaging the values.
pub fn get_data_from_ogg(path: &Path) -> Result<(Samples, SoundFileInfo), DecoderError> {
    println!("Reading file: {:?}", path);
    let decoder = Decoder::open(path)?;
    let info = decoder.info();

    let data: Vec<f32> = decoder
        .into_samples()?
        .collect::<Result<Vec<f32>, DecoderError>>()?;
    let data: Vec<f32> = data.chunks_exact(2).map(|x| (x[0] + x[1]) / 2.0).collect();

    let title = path
        .file_stem()
        .and_then(|x| x.to_str())
        .unwrap_or(&"file_name")
        .to_owned();
    let audio_total_time = data.len() as f32 / info.sample_rate() as f32;

    Ok((
        Samples {
            rate: info.sample_rate(),
            data,
        },
        SoundFileInfo {
            title,
            audio_total_time,
        },
    ))
}

/// Gets a series of `Note` from a `Samples` object.
pub fn get_notes_from_samples(
    samples: Samples,
    difficulty_limit: f32,
) -> Result<Vec<Note>, SpectrumAnalyzerError> {
    println!("Getting notes...");

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

        if val > 0.001 && val > difficulty * difficulty_limit {
            result.push(Note { freq, val, time });
        }
    }

    println!(
        "Generate {} notes...with difficulty limit: {difficulty_limit:.2}",
        result.len()
    );

    let count_min_limit = (samples.data.len() as f32 * 2.5 / samples.rate as f32) as usize;
    let count_max_limit = (samples.data.len() as f32 * 3.5 / samples.rate as f32) as usize;
    if result.len() < count_min_limit {
        result = get_notes_from_samples(samples, difficulty_limit - 0.02)?;
    } else if result.len() > count_max_limit {
        result = get_notes_from_samples(samples, difficulty_limit + 0.02)?;
    }

    Ok(result)
}

/// Gets a series `Wall` from a note slice
pub fn get_walls_from_notes(notes: &[Note], level_difficulty: &LevelDifficulty) -> Vec<Wall> {
    let mut rng = rand::thread_rng();

    let mut prev_wall = None;
    let mut acc_coins = 0u8;
    notes
        .windows(3)
        .map(|notes| {
            let time2prev = notes[1].time - notes[0].time;
            let time2next = notes[2].time - notes[1].time;
            Wall::new(
                &notes[1],
                level_difficulty,
                &mut rng,
                time2prev,
                time2next,
                &mut prev_wall,
                &mut acc_coins,
            )
        })
        .collect()
}

/// Generates the output yml string content
pub fn gen_yml(
    SoundFileInfo {
        title,
        audio_total_time,
    }: &SoundFileInfo,
    LevelDifficulty {
        speed,
        difficulty,
        min_interval: _,
        max_consecutive_coins: _,
    }: &LevelDifficulty,
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
scenario: 3

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

    let mut rng = rand::thread_rng();

    for wall in walls {
        let time = format!("{:.2}", wall.0);
        let code = wall.to_code(&mut rng);

        result.push_str(
            format!("      - second: {time}\n        obj: {code}\n        track: 0\n").as_str(),
        );
    }

    result.push_str("...");
    result
}
