# Sonic Spectra

**Sonic Spectra** is a real-time audio visualizer project built in Rust. It uses the GTK4 library for the graphical user interface and RustFFT for performing Fast Fourier Transforms (FFT) on audio data. This project is currently in its initial stages and serves as a platform for exploring and learning the Rust programming language.

## Features

- **Real-time Audio Visualization**: Visualizes audio input using various techniques, including frequency range visualizations and holographic glow effects.
- **Customizable Settings**: Users can configure visualization parameters such as frequency ranges and color mappings.
- **Asynchronous Audio Processing**: Uses Tokio for managing asynchronous audio streaming and processing.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (stable version)
- [GTK4](https://www.gtk.org/) (ensure you have the required development libraries installed)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/sonic_spectra.git
   cd sonic_spectra

