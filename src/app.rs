use egui::mutex::Mutex;
use hecs::World;
use std::sync::Arc;

use crate::Metadata;

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

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe template");
            ui.label(format!("Total iterations: {}", metadata.total_iterations));
            ui.label(format!(
                "Previous execution time: {:?}",
                metadata.previous_execution_time
            ));
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.label("Uwu");
        });
    }
}
