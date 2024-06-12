use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RePathSettings {
    pub navmesh_filename: String,
    pub precompute_radius: f64,
    pub total_precompute_pairs: usize,
    pub cache_capacity: usize,
    pub use_precomputed_cache: bool,
}
