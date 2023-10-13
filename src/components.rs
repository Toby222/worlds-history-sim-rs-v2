#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub total_iterations: usize,
    pub iterations_last_second: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct WorldTextureHandle(pub eframe::egui::TextureHandle);
