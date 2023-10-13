#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Metadata {
    pub(crate) total_iterations: usize,
    pub(crate) iterations_last_second: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct WorldTextureHandle(pub(crate) eframe::egui::TextureHandle);
