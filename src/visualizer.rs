use gtk::cairo::Context;
use gtk4 as gtk;
use rustfft::num_complex::Complex32;

/// A trait defining a generic interface for visualizers that can draw audio data
/// on a graphical context using FFT (Fast Fourier Transform) data.
///
/// This trait is intended to be implemented by various types of audio visualizers
/// to allow flexible rendering of audio data in real-time applications.
///
/// # Required Method
/// - `draw`: Renders the visualizer using FFT data for both left and right audio channels.
pub trait Visualizer: Send + Sync {
    /// Draws the visualizer's output onto a given graphical context (`cr`) using FFT data.
    ///
    /// # Arguments
    /// - `width`: The width of the drawing area in pixels.
    /// - `height`: The height of the drawing area in pixels.
    /// - `fft_left`: FFT data for the left audio channel, represented as a slice of complex values.
    /// - `fft_right`: FFT data for the right audio channel, represented as a slice of complex values.
    /// - `cr`: The Cairo drawing context used for rendering.
    /// - `previous_heights_left`: A mutable vector storing the previous heights of bars (or other elements)
    ///   for the left channel, used for smooth transitions or interpolation.
    /// - `previous_heights_right`: A mutable vector storing the previous heights of bars for the right channel.
    ///
    /// # Description
    /// Implementations of this function should use the FFT data (`fft_left` and `fft_right`)
    /// to create a visual representation of the audio spectrum. The `previous_heights_left`
    /// and `previous_heights_right` vectors allow the visualizer to retain state between
    /// frames, enabling smoother transitions by interpolating between previous and current
    /// frame values.
    fn draw(
        &self,
        width: i32,
        height: i32,
        fft_left: &[Complex32],
        fft_right: &[Complex32],
        cr: &Context,
        previous_heights_left: &mut Vec<f32>,
        previous_heights_right: &mut Vec<f32>,
    );
}
