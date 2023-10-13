use eframe::{
    egui::{
        self, load::SizedTexture, mutex::RwLock, paint_texture_at, ImageOptions, LayerId, Pos2,
        Rect, Slider,
    },
    epaint::Color32,
};
use hecs::World;
use std::{sync::Arc, time::Duration};

use crate::{
    planet::{Planet, PlanetUpdated},
    Metadata, Separator, WorldTextureHandle,
};

pub(crate) struct SimulatorApp {
    world: Arc<RwLock<World>>,
    new_planet_size: (usize, usize),
}

impl SimulatorApp {
    pub(crate) fn new(world: Arc<RwLock<World>>) -> Self {
        Self {
            world,
            new_planet_size: (640, 400),
        }
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

        let painter = ctx.layer_painter(LayerId::background());
        let tex: SizedTexture = SizedTexture::from_handle(&world_texture_handle.0);
        let rect = Rect::from_two_pos(Pos2::ZERO, Pos2::ZERO + tex.size);
        paint_texture_at(
            &painter,
            rect,
            &ImageOptions {
                bg_fill: Color32::RED,
                ..Default::default()
            },
            &tex,
        );

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let max_size = ctx.screen_rect().size();
                let max_width = max_size.x as usize;
                let max_height = max_size.y as usize;
                self.new_planet_size.0 = self.new_planet_size.0.clamp(0, max_width);
                self.new_planet_size.1 = self.new_planet_size.1.clamp(0, max_height);
                ui.add(Slider::new(&mut self.new_planet_size.0, 0..=max_width).text("Width"));
                ui.separator();
                ui.add(Slider::new(&mut self.new_planet_size.1, 0..=max_height).text("Height"));
                ui.separator();

                if ui.button("Generate new planet").clicked() {
                    let mut query = world.query::<(&mut Planet, &mut PlanetUpdated)>();
                    let (_, (planet, planet_updated)) =
                        query.iter().next().expect("Planet missing");
                    planet_updated.0 = true;
                    planet.generate(self.new_planet_size.0, self.new_planet_size.1);
                }
            });
        });

        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                ui.label("Iterations last second");
                ui.label(metadata.iterations_last_second.separated::<3, '.'>());

                ui.label("Total iterations:");
                ui.label(metadata.total_iterations.separated::<3, '.'>());
            });
        });

        if cfg!(debug_assertions) {
            egui::TopBottomPanel::bottom("debug_warning").show(ctx, |ui| {
                egui::warn_if_debug_build(ui);
            });
        }
        ctx.request_repaint_after(Duration::from_secs_f64(1.0));
    }
}
