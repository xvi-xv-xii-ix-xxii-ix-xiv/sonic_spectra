use crate::settings::Settings;
use gtk::cairo::Context;
use gtk4 as gtk;
use std::sync::Arc;

/// A structure representing the frequency grid used for visualizing audio data.
///
/// # Fields
/// - `settings`: A reference-counted `Settings` object that contains grid and FFT configurations.
pub struct FrequencyGrid {
    settings: Arc<Settings>, // Stores settings related to grid and FFT configuration
}

impl FrequencyGrid {
    /// Creates a new `FrequencyGrid` instance.
    ///
    /// # Arguments
    /// - `settings`: Shared settings containing grid and FFT configurations.
    ///
    /// # Returns
    /// - A new `FrequencyGrid` instance configured with the provided settings.
    pub fn new(settings: Arc<Settings>) -> Self {
        FrequencyGrid { settings }
    }

    /// Draws the frequency grid on a drawing area, including horizontal and vertical lines.
    ///
    /// # Arguments
    /// - `cr`: The Cairo `Context` used for drawing.
    /// - `width`: The width of the drawing area.
    /// - `height`: The height of the drawing area.
    ///
    /// This function draws a grid with horizontal lines and vertical frequency markers for both left
    /// and right audio channels. The grid appearance is customizable through the settings.
    pub fn draw(&self, cr: &Context, width: f64, height: f64) {
        let grid_settings = &self.settings.grid; // Access grid-related settings
        let fft_settings = &self.settings.fft; // Access FFT-related settings

        // Set the color and line thickness for the horizontal grid lines
        cr.set_source_rgba(
            grid_settings.color_horizontal[0],
            grid_settings.color_horizontal[1],
            grid_settings.color_horizontal[2],
            grid_settings.alpha,
        );
        cr.set_line_width(grid_settings.line_width); // Set grid line thickness

        // Draw horizontal grid lines based on the number of lines specified in settings
        for i in 0..grid_settings.lines {
            let y = height * (i as f64 / grid_settings.lines as f64);
            cr.move_to(0.0, y);
            cr.line_to(width, y);
        }
        cr.stroke().expect("Failed to draw horizontal grid lines");

        // Set half of the width as a reference for drawing symmetrical lines
        let half_width = width / 2.0;

        // Calculate the scale factor for the X-axis based on the maximum frequency
        let scale_factor = half_width as f32 / fft_settings.max_frequency;

        // Exit if there are no frequencies set in the FFT settings
        if let Some(frequencies) = &fft_settings.frequencies {
            // Draw vertical frequency lines for both left and right audio channels
            for &frequency in frequencies.iter() {
                let x_position = frequency * scale_factor;

                // Draw lines for the left channel (red color)
                if x_position >= 0.0 && x_position <= half_width as f32 {
                    cr.set_source_rgba(
                        grid_settings.color_left[0],
                        grid_settings.color_left[1],
                        grid_settings.color_left[2],
                        grid_settings.alpha,
                    );
                    cr.set_line_width(1.0);
                    cr.move_to(half_width - x_position as f64, 0.0); // Line on the left of the center
                    cr.line_to(half_width - x_position as f64, height);
                    cr.stroke().expect("Failed to draw left channel grid lines");
                }

                // Draw lines for the right channel (green color)
                if x_position >= 0.0 && x_position <= half_width as f32 {
                    cr.set_source_rgba(
                        grid_settings.color_right[0],
                        grid_settings.color_right[1],
                        grid_settings.color_right[2],
                        grid_settings.alpha,
                    );
                    cr.set_line_width(1.0);
                    cr.move_to(half_width + x_position as f64, 0.0); // Line on the right of the center
                    cr.line_to(half_width + x_position as f64, height);
                    cr.stroke()
                        .expect("Failed to draw right channel grid lines");
                }
            }
        } else {
            eprintln!("Frequencies are not set in FFT settings");
        }
    }
}
