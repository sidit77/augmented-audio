use vst::buffer::AudioBuffer;
use vst::host::HostBuffer;

use audio_processor_traits::AudioProcessorSettings;

/// Handles conversion from CPAL buffers to VST buffers
pub struct CpalVstBufferHandler {
    audio_settings: AudioProcessorSettings,
    input_buffer: Vec<Vec<f32>>,
    output_buffer: Vec<Vec<f32>>,
    host_buffer: HostBuffer<f32>,
}

impl CpalVstBufferHandler {
    /// Create a buffer handler
    pub fn new(audio_settings: AudioProcessorSettings) -> Self {
        let num_channels = audio_settings.input_channels();
        let buffer_size = audio_settings.block_size();

        let input_buffer = Self::allocate_buffer(num_channels, buffer_size);
        let output_buffer = Self::allocate_buffer(num_channels, buffer_size);
        let host_buffer = HostBuffer::new(num_channels, num_channels);
        log::info!("Buffer handler: num_channels={}", num_channels);

        CpalVstBufferHandler {
            audio_settings,
            input_buffer,
            output_buffer,
            host_buffer,
        }
    }

    /// Prepare the handler given changed audio settings
    pub fn prepare(&mut self, audio_settings: &AudioProcessorSettings) {
        self.audio_settings = *audio_settings;

        let num_channels = audio_settings.input_channels();
        let buffer_size = audio_settings.block_size();

        self.input_buffer = Self::allocate_buffer(num_channels, buffer_size);
        self.output_buffer = Self::allocate_buffer(num_channels, buffer_size);
        self.host_buffer = HostBuffer::new(num_channels, num_channels);
    }

    /// Process cpal input samples
    pub fn process(&mut self, data: &[f32]) {
        for (sample_index, frame) in data
            .chunks(self.audio_settings.input_channels())
            .enumerate()
        {
            for (channel, sample) in frame.iter().enumerate() {
                self.input_buffer[channel][sample_index] = *sample;
            }
        }
    }

    /// Get the VST audio buffer
    pub fn get_audio_buffer(&mut self) -> AudioBuffer<f32> {
        self.host_buffer
            .bind(&self.input_buffer, &mut self.output_buffer)
    }

    fn allocate_buffer(channels: usize, buffer_size: u32) -> Vec<Vec<f32>> {
        let mut buffer = Vec::new();
        buffer.reserve(channels);
        for _ in 0..channels {
            let channel_buffer = vec![0.0; buffer_size as usize];
            buffer.push(channel_buffer);
        }
        buffer
    }
}
