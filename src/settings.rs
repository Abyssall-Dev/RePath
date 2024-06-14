use serde::{Serialize, Deserialize};

/// Configuration settings for the RePathfinder.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RePathSettings {
    /// The filename of the navigation mesh in Wavefront OBJ format.
    pub navmesh_filename: String,
    
    /// The radius within which to precompute paths between nodes.
    /// Higher values will result in longer precomputation times but faster pathfinding for long distances.
    pub precompute_radius: f64,
    
    /// The total number of node pairs for which paths will be precomputed.
    /// Higher values will result in longer precomputation times but more efficient pathfinding.
    pub total_precompute_pairs: usize,
    
    /// The capacity of the LRU cache for storing precomputed paths.
    /// Higher values allow more paths to be stored but will use more memory.
    pub cache_capacity: usize,
    
    /// Whether to use the precomputed cache for pathfinding.
    /// Set to false to disable the use of precomputed paths.
    pub use_precomputed_cache: bool,
}
