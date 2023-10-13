use eframe::egui::{Color32, ColorImage};
use grid::Grid;

#[derive(Default, Clone, Copy, Debug)]
pub(crate) struct Cell {
    pub(crate) temperature: f32,
    pub(crate) humidity: f32,
    pub(crate) elevation: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct PlanetUpdated(pub(crate) bool);

#[derive(Clone, Debug)]
pub(crate) struct Planet {
    pub(crate) cells: Grid<Cell>,
}

impl Planet {
    pub(crate) fn new(rows: usize, cols: usize) -> Self {
        Planet {
            cells: Grid::new(rows, cols),
        }
    }

    pub(crate) fn empty() -> Self {
        Self::new(0, 0)
    }

    pub(crate) fn generate(&mut self, width: usize, height: usize) {
        log::debug!("Generating world of size {}x{}", width, height);
        let width_f32 = width as f32;
        let height_f32 = height as f32;

        self.cells = Grid::new(height, width);

        for y in 0..self.cells.rows() {
            for (x, cell) in self.cells.iter_row_mut(y).enumerate() {
                cell.humidity = x as f32 / width_f32;
                debug_assert!(
                    cell.humidity >= 0.0 && cell.humidity <= 1.0,
                    "cell.humidity needs to be a scalar between 0.0 and 1.0. Is {} ({}, {}, {})",
                    cell.humidity,
                    x,
                    height_f32,
                    height,
                );

                cell.temperature =
                    1.0 - f32::sin((y as f32 / height_f32) * std::f32::consts::PI).clamp(0.0, 1.0); // clamp because of rounding errors
                debug_assert!(
                    cell.temperature >= 0.0 && cell.temperature <= 1.0,
                    "cell.temperature needs to be a scalar between 0.0 and 1.0. Is {}",
                    cell.temperature
                );

                cell.elevation = 0.0;
                debug_assert!(
                    cell.elevation >= 0.0 && cell.elevation <= 1.0,
                    "cell.elevation needs to be a scalar between 0.0 and 1.0. Is {}",
                    cell.elevation
                );
            }
        }
    }
}

impl From<&Planet> for ColorImage {
    fn from(planet: &Planet) -> Self {
        ColorImage {
            size: [planet.cells.size().1, planet.cells.size().0],
            pixels: planet
                .cells
                .iter()
                .map(|cell| {
                    let temperature = cell.temperature;
                    let humidity = cell.humidity;
                    let elevation = cell.elevation;
                    Color32::from_rgb(
                        (temperature * 255.0) as u8,
                        (humidity * 255.0) as u8,
                        (elevation * 255.0) as u8,
                    )
                })
                .collect(),
        }
    }
}
