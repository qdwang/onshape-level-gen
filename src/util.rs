//! This file contains general functions.
//!
use super::*;
use creak::{Decoder, DecoderError};
use spectrum_analyzer::error::*;
use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

/// Get the one channel `Vec<f32>` data from an ogg file.
///
/// The data from 2 channels will be combined into one channel by averaging the values.
pub fn get_data_from_ogg(path: &str) -> Result<Samples, DecoderError> {
    let decoder = Decoder::open(path)?;
    let info = decoder.info();

    let data: Vec<f32> = decoder
        .into_samples()?
        .collect::<Result<Vec<f32>, DecoderError>>()?;
    let data: Vec<f32> = data.chunks_exact(2).map(|x| (x[0] + x[1]) / 2.0).collect();

    Ok(Samples {
        rate: info.sample_rate(),
        data,
    })
}

/// Get a series of `Note` from a `Samples` object.
/// 
/// The core shapes generating algorithm is inside this function.
pub fn get_notes_from_samples(samples: Samples) -> Result<Vec<Note>, SpectrumAnalyzerError> {
    const FFT_SIZE: usize = 4096;
    const MIN_HZ : f32 = 50.;
    const MAX_HZ : f32 = 6000.;

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
