use crate::settings::Settings;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;

/// Structure to hold the audio data buffers for left and right channels.
pub struct AudioData {
    pub left_buffer: Vec<f32>,
    pub right_buffer: Vec<f32>,
}

impl AudioData {
    /// Creates a new `AudioData` instance with buffers initialized to zero.
    ///
    /// # Arguments
    /// - `fft_size`: Size of the FFT, which determines the buffer length for each channel.
    ///
    /// # Returns
    /// - `AudioData` instance with zero-initialized buffers.
    pub fn new(fft_size: usize) -> Self {
        AudioData {
            left_buffer: vec![0.0; fft_size],
            right_buffer: vec![0.0; fft_size],
        }
    }
}

/// Starts an audio input stream to capture audio data for FFT processing.
///
/// # Arguments
/// - `audio_data`: A thread-safe, shared reference to `AudioData` where captured audio samples will be stored.
/// - `settings`: A shared reference to `Settings` containing FFT and audio configuration details.
pub fn start_audio_stream(audio_data: Arc<Mutex<AudioData>>, settings: Arc<Settings>) {
    let fft_size = settings.fft.size;

    thread::spawn(move || {
        // Initialize the CPAL host to interface with audio input devices
        let host = cpal::default_host();
        let device = match host.default_input_device() {
            Some(d) => d,
            None => {
                eprintln!("No available input devices.");
                return;
            }
        };

        // Retrieve the device’s default input configuration
        let config = match device.default_input_config() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to retrieve input configuration: {}", e);
                return;
            }
        };

        let channels = config.channels(); // Number of audio channels (e.g., 1 for mono, 2 for stereo)
        let config: cpal::StreamConfig = config.into(); // Convert configuration to `StreamConfig` format

        // Attempt to build an audio input stream with the specified settings
        let stream = match device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut audio = audio_data.lock().unwrap(); // Lock the audio data for safe access
                for i in 0..fft_size {
                    let idx = i * channels as usize;
                    if idx < data.len() {
                        // Handle mono or stereo channel data appropriately
                        if channels == 1 {
                            audio.left_buffer[i] = data[idx];
                            audio.right_buffer[i] = data[idx];
                        } else if channels >= 2 {
                            audio.left_buffer[i] = data[idx];
                            audio.right_buffer[i] = data[idx + 1];
                        }
                    } else {
                        // Fill with zeroes if data is insufficient
                        audio.left_buffer[i] = 0.0;
                        audio.right_buffer[i] = 0.0;
                    }
                }
            },
            move |err| {
                eprintln!("Stream error: {}", err); // Error handling callback
            },
            None,
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to create stream: {}", e);
                return;
            }
        };

        // Start the stream
        if let Err(e) = stream.play() {
            eprintln!("Failed to start the stream: {}", e);
            return;
        }

        // Keep the thread alive as long as the program is running
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
