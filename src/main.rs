#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod components;
mod formatted_numbers;
mod planet;

use eframe::egui;
use egui::{mutex::RwLock, TextureHandle};
use hecs::World;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use app::SimulatorApp;
use components::{Metadata, WorldTextureHandle};
use formatted_numbers::Separator;
use planet::*;

fn main() -> color_eyre::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1600.0, 900.0].into()),
        ..Default::default()
    };

    let mut world = hecs::World::new();
    world.spawn((Metadata::default(),));
    world.spawn((Planet::empty(), PlanetUpdated(true)));
    let world = Arc::new(RwLock::new(world));
    let app = SimulatorApp::new(world.clone());

    let world_clone = world.clone();
    thread::Builder::new()
        .name("systems".into())
        .spawn(move || run_systems(world_clone))?;
    eframe::run_native(
        "Systems test",
        native_options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            cc.egui_ctx.style_mut(|style| {
                style.text_styles.iter_mut().for_each(|text_style| {
                    text_style.1.size *= 1.5;
                });
            });

            let world_texture_id = cc.egui_ctx.tex_manager().write().alloc(
                "World".into(),
                egui::ImageData::Color(Arc::new(egui::ColorImage::default())),
                egui::TextureOptions::NEAREST,
            );
            world.write().spawn((WorldTextureHandle(TextureHandle::new(
                cc.egui_ctx.tex_manager(),
                world_texture_id,
            )),));

            Box::new(app)
        }),
    )
    .expect("Failed to run eframe");
    Ok(())
}

fn run_systems(world: Arc<RwLock<World>>) -> ! {
    let start_time = Instant::now();
    let mut one_second_timer = Instant::now();
    let mut quarter_second_timer = Instant::now();
    let mut iterations_per_second = 0;

    loop {
        let mut world = world.write();

        let (_, metadata) = world
            .query_mut::<&mut Metadata>()
            .into_iter()
            .next()
            .expect("Iterations entity is missing");
        metadata.total_iterations = metadata.total_iterations.wrapping_add(1);

        iterations_per_second += 1;
        if one_second_timer.elapsed() >= Duration::from_secs(1) {
            log::debug!(
                "Total iterations: {}, actual time since start: {:?}, iterations last second: {}",
                metadata.total_iterations.separated::<3, '.'>(),
                start_time.elapsed(),
                iterations_per_second.separated::<3, '.'>(),
            );
            metadata.iterations_last_second = iterations_per_second;
            iterations_per_second = 0;
            one_second_timer = Instant::now();
        }

        if quarter_second_timer.elapsed() >= Duration::from_millis(250) {
            let (_, (planet, planet_updated)) = world
                .query_mut::<(&Planet, &mut PlanetUpdated)>()
                .into_iter()
                .next()
                .expect("Planet missing");
            let world_image = if planet_updated.0 {
                planet_updated.0 = false;
                Some(egui::ImageData::Color(Arc::new(planet.into())))
            } else {
                None
            };

            if let Some(image) = world_image {
                let (_, world_texture_handle) = world
                    .query_mut::<&mut WorldTextureHandle>()
                    .into_iter()
                    .next()
                    .expect("WorldTextureHandle missing");

                world_texture_handle
                    .0
                    .set(image, egui::TextureOptions::NEAREST);
            }

            quarter_second_timer = Instant::now();
        }
    }
}
