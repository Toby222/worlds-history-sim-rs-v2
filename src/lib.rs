#![warn(clippy::all, rust_2018_idioms)]

mod app;
use std::time::Duration;

pub use app::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub total_iterations: usize,
    pub previous_execution_time: Duration,
}
