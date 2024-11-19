use serde::{Serialize, Deserialize};
use crate::settings::RePathSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    #[serde(flatten)]
    pub settings: RePathSettings,
    pub precomputation_time: f32,
    pub pathfinding_time: f32,
    pub total_paths_precomputed: usize,
}

impl Metrics {
    pub fn new(settings: RePathSettings) -> Self {
        Metrics {
            settings,
            precomputation_time: 0.0,
            pathfinding_time: 0.0,
            total_paths_precomputed: 0,
        }
    }
}
