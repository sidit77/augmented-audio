pub struct TimeSignature {
    beats_per_bar: usize,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self { beats_per_bar: 4 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TempoEstimate {
    pub num_bars: usize,
    pub tempo: f32,
}

/// Performs tempo estimation into a `TimeSignature` by trying to divide a certain audio length by
/// a power of 2 number of bars between 1 and 256.
///
/// Returns the lowest number of bars that is in the 80-160bpm range.
///
/// This function can return very high tempo estimates and the consumer should ignore setting
/// unreasonable tempos.
pub fn estimate_tempo(
    time_signature: TimeSignature,
    sample_rate: f32,
    length_samples: usize,
) -> TempoEstimate {
    let beats_per_bar = time_signature.beats_per_bar as f32;
    let length_secs = (length_samples as f32) / sample_rate;

    let mut tempo_candidate = TempoEstimate {
        tempo: 0.0,
        num_bars: 0,
    };
    for i in 0..8 {
        let num_bars = 2u32.pow(i);
        let num_bars = num_bars as f32;
        let secs_per_bar = length_secs / num_bars;
        let secs_per_beat = secs_per_bar / beats_per_bar;

        tempo_candidate = TempoEstimate {
            tempo: (1.0 / secs_per_beat) * 60.0,
            num_bars: num_bars as usize,
        };
        if tempo_candidate.tempo >= 80.0 && tempo_candidate.tempo <= 160.0 {
            return tempo_candidate;
        }
    }

    tempo_candidate
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test_tempo_estimation_with_0to1hour() {
        let sample_rate = 44100.0;
        let max_duration_ms = (60 * 60) * 1000;
        for duration_ms in 2000..max_duration_ms {
            let length_samples = ((duration_ms as f32 / 1000.0) * sample_rate) as usize;
            let result = estimate_tempo(Default::default(), sample_rate, length_samples);
            assert!(
                result.tempo < 300.0,
                "tempo={} length={} duration_ms={}",
                result.tempo,
                length_samples,
                duration_ms
            );
        }
    }

    #[test]
    fn test_tempo_estimation() {
        let sample_rate = 44100.0;
        let tempo = 120.0;
        let secs_per_beat = 1.0 / (tempo / 60.0);
        let length_samples = sample_rate * secs_per_beat * 16.0;

        let result = estimate_tempo(Default::default(), sample_rate, length_samples as usize);
        assert_eq!(result.num_bars, 4);
        assert_eq!(result.tempo, 120.0);
    }
}
