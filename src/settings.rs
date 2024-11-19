use serde::{Serialize, Deserialize};

/// Configuration settings for the RePathfinder.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RePathSettings {
    /// The filename of the navigation mesh in Wavefront OBJ format.
    pub navmesh_filename: String,

    /// The radius within which to precompute paths between nodes.
    /// Higher values will result in longer precomputation times but faster pathfinding for long distances.
    pub precompute_radius: f32,

    /// The total number of node pairs for which paths will be precomputed.
    /// Higher values will result in longer precomputation times but more efficient pathfinding.
    pub total_precompute_pairs: usize,

    /// Whether to use the precomputed cache for pathfinding.
    /// Set to false to disable the use of precomputed paths.
    pub use_precomputed_cache: bool,
}
