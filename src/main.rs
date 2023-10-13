#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    error::Error,
    fmt::Display,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use egui::mutex::Mutex;
use hecs::World;
use worlds_history_sim_rs::*;

#[derive(Debug)]
enum SimulationError {
    ThreadSpawnError(std::io::Error),
    EframeError(eframe::Error),
}

impl Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::ThreadSpawnError(err) => write!(f, "Thread spawn error: {}", err),
            SimulationError::EframeError(err) => write!(f, "Eframe error: {}", err),
        }
    }
}

impl Error for SimulationError {}

fn main() -> Result<(), SimulationError> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };

    let mut world = hecs::World::new();
    world.spawn((worlds_history_sim_rs::Metadata::default(),));
    let world = Arc::new(Mutex::new(world));
    let app = SimulatorApp::new(world.clone());

    thread::Builder::new()
        .name("systems".into())
        .spawn(move || run_systems(world))
        .map_err(|err| SimulationError::ThreadSpawnError(err))?;
    eframe::run_native(
        "Systems test",
        native_options,
        Box::new(move |_cc| Box::new(app)),
    )
    .map_err(|err| SimulationError::EframeError(err))?;
    Ok(())
}

fn run_systems(world: Arc<Mutex<World>>) -> ! {
    let start_time = Instant::now();
    let mut prev_iteration = Instant::now();

    let mut one_second_timer = Instant::now();
    loop {
        let mut world = world.lock();

        let (_entity, metadata) = world
            .query_mut::<&mut Metadata>()
            .into_iter()
            .next()
            .expect("Iterations entity is missing");
        metadata.total_iterations = metadata.total_iterations.wrapping_add(1);
        metadata.previous_execution_time = prev_iteration.elapsed();

        if one_second_timer.elapsed() >= Duration::from_secs(1) {
            log::debug!(
                "Total iterations: {}, actual time since start: {:?}, iterations per second: {}",
                metadata.total_iterations,
                start_time.elapsed(),
                metadata.total_iterations as f64 / start_time.elapsed().as_secs_f64()
            );
            one_second_timer = Instant::now();
        }

        prev_iteration = Instant::now();
    }
}
