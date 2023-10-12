use hecs::{Entity, World};

pub struct SimulatorApp {
    world: World,
    iterations_entity: Entity,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Iterations(usize);

impl SimulatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut world = World::new();
        let iterations_entity = world.spawn((Iterations(0),));
        Self {
            world,
            iterations_entity,
        }
    }
}

impl eframe::App for SimulatorApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let iterations = self
            .world
            .query_one_mut::<&mut Iterations>(self.iterations_entity)
            .expect("Iterations entity missing");

        iterations.0 += 1;
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
            ui.label(iterations.0.to_string());
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
