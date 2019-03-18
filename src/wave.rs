use rodio::{self, Sample, Sink, Source};
use std::thread;
use std::time::Duration;
use std::usize;

pub enum WaveKind {
    Sine,
}

pub struct WaveConfig {
    pub hz: usize,
    pub phase_hz: usize,
    pub kind: WaveKind,
}

pub struct WaveSource {
    config: WaveConfig,
    channels: u16,
    sample_rate: u32,
    buffer: Vec<f32>,
    buffer_samples: usize,
    sample_it: usize,
    samples_total: usize,
}

use std::f64;
const PI_2: f64 = f64::consts::PI * 2.0f64;
impl WaveKind {
    fn get(&self) -> Box<Fn(f64) -> f64> {
        match self {
            WaveKind::Sine => Box::new(|t: f64| (t * PI_2).sin()),
        }
    }
}

impl WaveSource {
    pub fn new(config: WaveConfig, length: Duration, sample_rate: u32) -> Self {
        let sample_rate_f = sample_rate as f64;
        let hz_f = config.hz as f64;
        // samples per phase
        let one_phase_len: f64 = sample_rate_f / hz_f;
        let one_phase_len_u = one_phase_len as usize;

        let mut buffer = vec![0f32; one_phase_len_u];
        let eq = config.kind.get();
        let mut i = 0usize;
        let mut t = 0f64;
        while i < one_phase_len_u {
            buffer[i] = eq(t / one_phase_len) as f32;
            i += 1usize;
            t += 1f64;
        }

        let total_samples = ((length.as_millis() * sample_rate as u128) / 1000) as usize;

        WaveSource {
            config: config,
            channels: 1,
            sample_rate: sample_rate,
            buffer: buffer,
            buffer_samples: one_phase_len_u,
            sample_it: 0usize,
            samples_total: total_samples,
        }
    }
}

impl Iterator for WaveSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_it < self.samples_total {
            let buf_idx = self.sample_it % self.buffer_samples;
            self.sample_it += 1usize;
            Some(self.buffer.get(buf_idx).expect("in bounds").clone())
        } else {
            None
        }
    }
}

impl Source for WaveSource {
    fn current_frame_len(&self) -> Option<usize> {
        if self.sample_it < self.samples_total {
            Some(self.samples_total - self.sample_it)
        } else {
            None
        }
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(
            (self.samples_total as f64 / self.sample_rate as f64 * 1000f64) as u64,
        ))
    }
}
