use eframe::egui::{
    self, load::SizedTexture, mutex::RwLock, paint_texture_at, ImageOptions, Pos2, Rect,
};
use hecs::World;
use std::{sync::Arc, time::Duration};

use crate::{Metadata, Separator, WorldTextureHandle};

pub struct SimulatorApp {
    world: Arc<RwLock<World>>,
}

impl SimulatorApp {
    pub fn new(world: Arc<RwLock<World>>) -> Self {
        Self { world }
    }
}

impl eframe::App for SimulatorApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let world = self.world.read();

        let mut query = world.query::<&mut Metadata>();
        let (_, metadata) = query.iter().next().expect("Metadata missing");
        let mut query = world.query::<&WorldTextureHandle>();
        let (_, world_texture_handle) = query.iter().next().expect("WorldTextureHandle missing");

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
                ui.label(metadata.total_iterations.separated::<3, '.'>());
                ui.end_row();

                ui.label("Iterations last second");
                ui.label(metadata.iterations_last_second.separated::<3, '.'>());
                ui.end_row();
            });

            let tex: SizedTexture = SizedTexture::from_handle(&world_texture_handle.0);
            let (_, painter) = ui.allocate_painter(tex.size * 2.0, egui::Sense::hover());
            paint_texture_at(
                &painter,
                Rect::from_two_pos(Pos2::ZERO, Pos2::from([tex.size.x, tex.size.y])),
                &ImageOptions::default(),
                &tex,
            );
        });

        if cfg!(debug_assertions) {
            egui::TopBottomPanel::bottom("debug_warning").show(ctx, |ui| {
                egui::warn_if_debug_build(ui);
            });
        }
        ctx.request_repaint_after(Duration::from_secs_f64(1.0 / 144.0));
    }
}
