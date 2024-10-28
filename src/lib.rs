use crate::frequency_holographic_glow_visualizer::HolographicGlowVisualizer;
use crate::frequency_range_visualizer::FrequencyRangeVisualizer;
use crate::settings::Settings;
use gtk::prelude::*;
use gtk::{gdk, Application, ApplicationWindow, CssProvider, DrawingArea};
use gtk4 as gtk;
use rustfft::FftPlanner;
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch;

mod audio;
mod fft_utils;
mod frequency_holographic_glow_visualizer;
mod frequency_range_visualizer;
mod grid;
mod settings;
mod visualizer;

const APP_ID: &str = "com.sonic_spectra";

/// Run the main application loop with the visualizer setup.
///
/// # Returns
/// - `Result` with no value if the program runs successfully, or an error if initialization fails.
pub fn run_application() -> Result<(), Box<dyn std::error::Error>> {
    let _rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let settings = Arc::new(Settings::new());
    let application = Application::builder().application_id(APP_ID).build();
    let (tx, rx) = watch::channel(());

    let audio_data = Arc::new(Mutex::new(audio::AudioData::new(settings.fft.size)));
    audio::start_audio_stream(audio_data.clone(), settings.clone());

    application.connect_activate(move |app| {
        if let Ok((window, drawing_area)) = load_ui(app) {
            if let Ok(css_provider) = load_css() {
                setup_css(&css_provider);
                initialize_visualizer(
                    &drawing_area,
                    audio_data.clone(),
                    settings.clone(),
                    tx.clone(),
                );
                setup_window_controls(&window, tx.clone());
                window.present();
                schedule_redraw(&drawing_area);
            } else {
                eprintln!("Error loading CSS.");
            }
        } else {
            eprintln!("Error loading UI.");
        }
    });

    handle_exit(rx.clone());

    application.run();

    Ok(())
}

/// Load the UI components from the specified resource file.
fn load_ui(
    application: &Application,
) -> Result<(ApplicationWindow, DrawingArea), Box<dyn std::error::Error>> {
    let builder = gtk::Builder::from_file("resources/ui/main.ui");
    let window: ApplicationWindow = builder
        .object("main_window")
        .ok_or("Failed to find main_window in UI file.")?;
    let drawing_area: DrawingArea = builder
        .object("drawing_area")
        .ok_or("Failed to find drawing_area in UI file.")?;

    window.set_application(Some(application));
    Ok((window, drawing_area))
}

/// Load CSS styling for the application.
fn load_css() -> Result<CssProvider, Box<dyn std::error::Error>> {
    let css_provider = CssProvider::new();
    let css_data = fs::read_to_string("resources/style.css")?;
    std::panic::catch_unwind(|| {
        css_provider.load_from_data(&css_data);
    })
    .map_err(|_| Box::<dyn std::error::Error>::from("Failed to load CSS due to panic"))?;
    Ok(css_provider)
}

/// Apply the CSS styling to the GTK display.
fn setup_css(css_provider: &CssProvider) {
    let display = gdk::Display::default().expect("Failed to get default display");
    gtk::style_context_add_provider_for_display(
        &display,
        css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Initialize and configure the visualizer for drawing.
fn initialize_visualizer(
    drawing_area: &DrawingArea,
    audio_data: Arc<Mutex<audio::AudioData>>,
    settings: Arc<Settings>,
    tx: watch::Sender<()>,
) {
    let planner = Arc::new(Mutex::new(FftPlanner::new()));
    let visualizer_type = "frequency";

    let visualizer: Box<dyn visualizer::Visualizer> = match visualizer_type {
        "frequency" => Box::new(FrequencyRangeVisualizer::new(settings.clone())),
        "holographic_glow" => Box::new(HolographicGlowVisualizer::new(settings.clone())),
        _ => Box::new(FrequencyRangeVisualizer::new(settings.clone())),
    };

    let num_bars = settings.fft.size / 2;
    let mut previous_heights_left = vec![0.0; num_bars];
    let mut previous_heights_right = vec![0.0; num_bars];
    let grid = Arc::new(grid::FrequencyGrid::new(settings.clone()));

    let drawing_area_clone = drawing_area.clone();
    let audio_data_clone = audio_data.clone();
    let planner_clone = planner.clone();
    let settings_clone = settings.clone();
    let grid_clone = grid.clone();

    drawing_area.set_draw_func(move |_widget, cr, _, _| {
        let width = drawing_area_clone.width() as f64;
        let height = drawing_area_clone.height() as f64;

        let audio = audio_data_clone.lock().unwrap();
        let input_left: Vec<f32> = audio.left_buffer.clone();
        let input_right: Vec<f32> = audio.right_buffer.clone();

        let mut input_left_clone: Vec<rustfft::num_complex::Complex32> = input_left
            .iter()
            .map(|&x| rustfft::num_complex::Complex32::new(x, 0.0))
            .collect();
        let mut input_right_clone: Vec<rustfft::num_complex::Complex32> = input_right
            .iter()
            .map(|&x| rustfft::num_complex::Complex32::new(x, 0.0))
            .collect();

        let fft_left = planner_clone
            .lock()
            .unwrap()
            .plan_fft_forward(settings_clone.fft.size);
        let fft_right = planner_clone
            .lock()
            .unwrap()
            .plan_fft_forward(settings_clone.fft.size);

        fft_left.process(&mut input_left_clone);
        fft_right.process(&mut input_right_clone);

        grid_clone.draw(cr, width, height);
        visualizer.draw(
            width as i32,
            height as i32,
            &input_left_clone,
            &input_right_clone,
            cr,
            &mut previous_heights_left,
            &mut previous_heights_right,
        );
    });
}

/// Set up window controls for key press handling and application exit.
fn setup_window_controls(window: &ApplicationWindow, tx: watch::Sender<()>) {
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == gdk::Key::Q {
            let _ = tx.send(());
            gtk::glib::Propagation::Proceed
        } else {
            gtk::glib::Propagation::Stop
        }
    });
    window.add_controller(key_controller);
}

/// Schedule redraw events for smooth animation.
fn schedule_redraw(drawing_area: &DrawingArea) {
    let drawing_area_clone = drawing_area.clone(); // Clone the DrawingArea
    gtk::glib::timeout_add_local(Duration::from_millis(30), move || {
        drawing_area_clone.queue_draw(); // Use the clone inside the closure
        gtk::glib::ControlFlow::Continue
    });
}

/// Handle application exit on receiving a shutdown signal.
fn handle_exit(rx: watch::Receiver<()>) {
    std::thread::spawn(move || {
        let mut rx = rx.clone();
        futures::executor::block_on(async {
            let _ = rx.changed().await;
            println!("Exiting the program...");
            std::process::exit(0);
        });
    });
}
