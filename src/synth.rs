use std::f32::consts::PI;
use rodio::source::Source;

const SAMPLE_RATE: u32 = 48000; // The sample rate of the audio in Hz.

// An infinite source that produces a sine.
// Always has a rate of 48kHz and one channel.
#[derive(Clone, Debug)]
pub struct Synth {
    freq: f32,
    num_sample: usize,
}

impl Synth {
    // The frequency of the sine.
    pub fn new(freq: f32) -> Synth {
        Synth {
            freq: freq,
            num_sample: 0,
        }
    }
}

impl Iterator for Synth {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / SAMPLE_RATE as f32;
        Some(value.sin().signum())
    }
}

impl Source for Synth {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}