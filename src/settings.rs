use serde::Deserialize;
use std::fs;

/// FFT (Fast Fourier Transform) settings used for audio processing.
///
/// # Fields
/// - `size`: The number of samples in the FFT, controlling frequency resolution.
/// - `sample_rate`: The sample rate of the audio, in Hz.
/// - `min_frequency`: The minimum frequency for visualization, in Hz.
/// - `max_frequency`: The maximum frequency for visualization, in Hz.
/// - `frequencies`: An optional list of specific frequencies for grid visualization.
#[derive(Deserialize)]
pub struct FFTSettings {
    pub size: usize,
    pub sample_rate: f32,
    pub min_frequency: f32,
    pub max_frequency: f32,
    pub frequencies: Option<Vec<f32>>, // Optional field for custom frequencies
}

/// Visualizer settings that control the appearance and behavior of the visualizer.
///
/// # Fields
/// - `gain`: Amplification factor for the visualized data.
/// - `scale_factor`: Factor to scale visual elements on the screen.
/// - `interpolation_factor`: Factor controlling interpolation for smoother animations.
/// - `alpha`: Opacity level of visual elements.
/// - `smooth_factor`: Smoothing factor to reduce visual jitter.
#[derive(Deserialize)]
pub struct VisualizerSettings {
    pub gain: f32,
    pub scale_factor: f32,
    pub interpolation_factor: f32,
    pub alpha: f32,
    pub smooth_factor: f32,
}

/// Grid settings for configuring the frequency grid in the visualizer.
///
/// # Fields
/// - `lines`: The number of horizontal lines in the grid.
/// - `color_left`: RGB color for the left channel lines.
/// - `color_right`: RGB color for the right channel lines.
/// - `color_horizontal`: RGB color for horizontal grid lines.
/// - `alpha`: Transparency level for the grid lines.
/// - `line_width`: Width of each grid line.
#[derive(Deserialize)]
pub struct GridSettings {
    pub lines: usize,
    pub color_left: [f64; 3],
    pub color_right: [f64; 3],
    pub color_horizontal: [f64; 3],
    pub alpha: f64,
    pub line_width: f64,
}

/// Root settings structure containing all configuration settings, including FFT, visualizer,
/// and grid configurations.
#[derive(Deserialize)]
pub struct Settings {
    pub fft: FFTSettings,
    pub visualizer: VisualizerSettings,
    pub grid: GridSettings,
}

impl FFTSettings {
    /// Generates logarithmically spaced frequencies if they are not provided in the configuration.
    ///
    /// # Arguments
    /// - `num_frequencies`: The number of frequencies to generate.
    ///
    /// # Returns
    /// - A vector of `num_frequencies` logarithmically spaced frequencies between `min_frequency`
    ///   and `max_frequency`.
    ///
    /// This function is useful when a specific list of frequencies is not provided, ensuring that
    /// the visualizer has frequency markers spread across the audible range.
    pub fn generate_frequencies(&self, num_frequencies: usize) -> Vec<f32> {
        let log_min = self.min_frequency.log10();
        let log_max = self.max_frequency.log10();

        // Create a vector of frequencies spaced logarithmically between `min_frequency` and `max_frequency`
        (0..num_frequencies)
            .map(|i| {
                let fraction = i as f32 / (num_frequencies - 1) as f32;
                let log_freq = log_min + fraction * (log_max - log_min);
                10_f32.powf(log_freq)
            })
            .collect()
    }
}

impl Settings {
    /// Loads and initializes settings from a configuration file (`config.toml`).
    ///
    /// # Returns
    /// - A `Settings` instance populated with data from the configuration file.
    ///
    /// If the `frequencies` field in `fft` settings is `None`, this method will auto-generate
    /// 15 logarithmically spaced frequencies between `min_frequency` and `max_frequency`.
    pub fn new() -> Self {
        let config_str =
            fs::read_to_string("resources/config.toml").expect("Unable to read config file");
        let mut settings: Settings = toml::from_str(&config_str).expect("Invalid config format");

        // Generate frequencies if they are not set in the configuration
        if settings.fft.frequencies.is_none() {
            settings.fft.frequencies = Some(settings.fft.generate_frequencies(15));
            // Generate 15 logarithmic frequencies
        }

        settings
    }
}
