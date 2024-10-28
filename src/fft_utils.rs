/// Calculates a color corresponding to a specific frequency range.
///
/// # Arguments
/// - `index`: The index of the current frequency bar.
/// - `total_bars`: The total number of frequency bars in the visualizer.
///
/// # Returns
/// - A tuple `(f32, f32, f32)` representing the RGB color values, each in the range [0.0, 1.0].
///
/// This function maps the frequency index to a hue value and converts it from HSL to RGB to
/// create a smooth gradient across the entire frequency range.
pub fn get_color_for_frequency(index: usize, total_bars: usize) -> (f32, f32, f32) {
    let frequency_ratio = index as f32 / total_bars as f32; // Calculate the position in the spectrum
    let hue = frequency_ratio * 360.0; // Map this position to a hue value (0-360 degrees)
    hsl_to_rgb(hue, 1.0, 0.5) // Convert HSL to RGB with full saturation and 50% lightness
}

/// Converts an HSL color value to RGB color space.
///
/// # Arguments
/// - `hue`: The hue angle in degrees (0-360), where different values represent distinct colors.
/// - `saturation`: The saturation level (0.0 to 1.0), where 1.0 is fully saturated and 0.0 is grayscale.
/// - `lightness`: The lightness level (0.0 to 1.0), where 0.5 gives pure color, 0.0 is black, and 1.0 is white.
///
/// # Returns
/// - A tuple `(f32, f32, f32)` representing the RGB color values, each in the range [0.0, 1.0].
///
/// This function performs calculations based on the HSL color model and handles different hue ranges
/// to generate the correct RGB output.
pub fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation; // Chroma: color intensity
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs()); // Intermediate value for hue transitions
    let m = lightness - c / 2.0; // Match the lightness level

    // Determine the RGB output based on hue range
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    // Adjust RGB values by adding `m` to each component
    (r + m, g + m, b + m)
}

/// Smoothly interpolates between a current and target value.
///
/// # Arguments
/// - `current`: The current value to be adjusted.
/// - `target`: The target value to move towards.
/// - `factor`: The interpolation factor (0.0 to 1.0), where 0.0 means no change and 1.0 means instant change.
///
/// # Returns
/// - `f32`: The interpolated value, closer to the target depending on the factor.
///
/// This function is useful for gradually adjusting values, like animating transitions or smoothing data.
pub fn interpolate(current: f32, target: f32, factor: f32) -> f32 {
    current + (target - current) * factor
}
