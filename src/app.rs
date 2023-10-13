use egui::mutex::Mutex;
use hecs::World;
use std::{sync::Arc, time::Duration};

use crate::{Metadata, Separator};

pub struct SimulatorApp {
    world: Arc<Mutex<World>>,
}

impl SimulatorApp {
    pub fn new(world: Arc<Mutex<World>>) -> Self {
        Self { world }
    }
}

impl eframe::App for SimulatorApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut world = self.world.lock();
        let (_, metadata) = world
            .query_mut::<&mut Metadata>()
            .into_iter()
            .next()
            .expect("Iterations entity missing");
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("metadata").num_columns(2).show(ui, |ui| {
                ui.label("Total iterations:");
                ui.label(metadata.total_iterations.separated::<3, ','>());
                ui.end_row();

                ui.label("Iterations last lescond");
                ui.label(metadata.iterations_last_second.separated::<3, ','>());
                ui.end_row();
            });

            ui.image(egui::include_image!("../assets/example.png"));
        });

        if cfg!(debug_assertions) {
            egui::TopBottomPanel::bottom("debug_warning").show(ctx, egui::warn_if_debug_build);
        }
        ctx.request_repaint_after(Duration::from_secs_f64(1.0 / 144.0));
    }
}
