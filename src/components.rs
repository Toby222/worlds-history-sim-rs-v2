#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub total_iterations: usize,
    pub iterations_last_second: usize,
}
