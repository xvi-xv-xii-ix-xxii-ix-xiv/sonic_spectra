use crate::fft_utils::{get_color_for_frequency, interpolate};
use crate::settings::Settings;
use crate::visualizer::Visualizer;
use gtk4::cairo::{Context, RadialGradient}; // Use gtk4::cairo
use rustfft::num_complex::Complex32;
use std::sync::Arc;

/// A visualizer that displays a holographic glow effect for audio visualization.
/// Generates colorful bars with a glow gradient effect based on FFT data for left
/// and right audio channels.
pub struct HolographicGlowVisualizer {
    settings: Arc<Settings>,
}

impl HolographicGlowVisualizer {
    /// Creates a new `HolographicGlowVisualizer` instance.
    ///
    /// # Arguments
    ///
    /// * `settings` - Shared application settings that control visualizer parameters.
    pub fn new(settings: Arc<Settings>) -> Self {
        HolographicGlowVisualizer { settings }
    }

    /// Calculates the minimum and maximum FFT indices based on frequency bounds.
    ///
    /// # Arguments
    ///
    /// * `fft_size` - The size of the FFT data array.
    ///
    /// # Returns
    ///
    /// A tuple containing the minimum and maximum indices for the specified frequency range.
    fn get_frequency_indices(&self, fft_size: usize) -> (usize, usize) {
        let fft_settings = &self.settings.fft;
        let min_freq = fft_settings.min_frequency;
        let max_freq = fft_settings.max_frequency;

        let min_index = (min_freq * fft_size as f32 / fft_settings.sample_rate) as usize;
        let max_index = (max_freq * fft_size as f32 / fft_settings.sample_rate) as usize;

        (min_index, max_index)
    }
}

impl Visualizer for HolographicGlowVisualizer {
    /// Renders the audio visualization with a holographic glow effect.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the visualization area.
    /// * `height` - The height of the visualization area.
    /// * `fft_left` - FFT data for the left audio channel.
    /// * `fft_right` - FFT data for the right audio channel.
    /// * `cr` - The Cairo context to draw on.
    /// * `previous_heights_left` - Stores previous heights of left channel bars for smooth animation.
    /// * `previous_heights_right` - Stores previous heights of right channel bars for smooth animation.
    fn draw(
        &self,
        width: i32,
        height: i32,
        fft_left: &[Complex32],
        fft_right: &[Complex32],
        cr: &Context,
        previous_heights_left: &mut Vec<f32>,
        previous_heights_right: &mut Vec<f32>,
    ) {
        let visual_settings = &self.settings.visualizer;
        let gain = visual_settings.gain;
        let scale_factor = visual_settings.scale_factor;
        let interpolation_factor = visual_settings.interpolation_factor;
        let alpha = visual_settings.alpha;

        let fft_size = fft_left.len();
        let (min_index, max_index) = self.get_frequency_indices(fft_size);

        // Select the FFT data range for visualization
        let fft_left = &fft_left[min_index..max_index];
        let fft_right = &fft_right[min_index..max_index];

        let num_bars = fft_left.len();
        let bar_width = width as f32 / (2.0 * num_bars as f32).max(1.0);

        // Draw the left channel with a glowing effect
        for i in 0..num_bars {
            let magnitude_left = fft_left[i].norm() * gain;
            let target_height_left = (magnitude_left + 1e-6).log10().max(0.0) * scale_factor;

            previous_heights_left[i] = interpolate(
                previous_heights_left[i],
                target_height_left,
                interpolation_factor,
            );

            let color_left = get_color_for_frequency(i, num_bars);

            // Create a radial gradient for the glowing effect
            let gradient = RadialGradient::new(
                width as f64 / 2.0,
                height as f64 / 2.0,
                0.0,
                width as f64 / 2.0,
                height as f64 / 2.0,
                previous_heights_left[i] as f64 * 2.0,
            );

            gradient.add_color_stop_rgba(
                0.0,
                color_left.0.into(),
                color_left.1.into(),
                color_left.2.into(),
                alpha as f64,
            );
            gradient.add_color_stop_rgba(
                1.0,
                color_left.0.into(),
                color_left.1.into(),
                color_left.2.into(),
                0.0,
            );

            let _ = cr.set_source(&gradient);

            let x = (num_bars as f32 - i as f32 - 1.0) * bar_width;
            let y = height as f32 - previous_heights_left[i];

            cr.rectangle(
                x as f64,
                y as f64,
                bar_width as f64,
                previous_heights_left[i] as f64,
            );
            cr.fill().unwrap();
        }

        // Draw the right channel with a glowing effect
        for i in 0..num_bars {
            let magnitude_right = fft_right[i].norm() * gain;
            let target_height_right = (magnitude_right + 1e-6).log10().max(0.0) * scale_factor;

            previous_heights_right[i] = interpolate(
                previous_heights_right[i],
                target_height_right,
                interpolation_factor,
            );

            let color_right = get_color_for_frequency(i, num_bars);

            // Create a radial gradient for the glowing effect
            let gradient = RadialGradient::new(
                width as f64 / 2.0,
                height as f64 / 2.0,
                0.0,
                width as f64 / 2.0,
                height as f64 / 2.0,
                previous_heights_right[i] as f64 * 2.0,
            );

            gradient.add_color_stop_rgba(
                0.0,
                color_right.0.into(),
                color_right.1.into(),
                color_right.2.into(),
                alpha as f64,
            );
            gradient.add_color_stop_rgba(
                1.0,
                color_right.0.into(),
                color_right.1.into(),
                color_right.2.into(),
                0.0,
            );

            let _ = cr.set_source(&gradient);

            let x = width as f32 - (num_bars as f32 - i as f32 - 1.0) * bar_width;
            let y = height as f32 - previous_heights_right[i];

            cr.rectangle(
                x as f64,
                y as f64,
                bar_width as f64,
                previous_heights_right[i] as f64,
            );
            cr.fill().unwrap();
        }
    }
}
