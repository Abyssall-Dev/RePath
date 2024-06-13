pub mod settings;
pub mod metrics;
pub mod node;
pub mod edge;
pub mod graph;
pub mod utils;
pub mod pathfinder;

pub use pathfinder::RePathfinder;

#[cfg(test)]
mod tests {
    use settings::RePathSettings;

    use super::*;

    #[test]
    fn test_pathfinding() {
        // Create a new RePathSettings instance with custom settings
        let settings = RePathSettings {
            navmesh_filename: "navmesh_varied.obj".to_string(), // Path to the navmesh file in Wavefront OBJ format
            precompute_radius: 20.0, // Higher this value, the longer it takes to precompute paths but faster pathfinding for long distances
            total_precompute_pairs: 100, // Higher this value, the longer it takes to precompute paths but faster pathfinding
            cache_capacity: 100, // Higher this value, the more paths can be stored in cache but more memory usage
            use_precomputed_cache: true, // Set to false to disable precomputation of paths
        };

        // Create a new RePathfinder instance
        let pathfinder = RePathfinder::new(settings);

        // Define start and end coordinates for pathfinding
        let start_coords = (0.0, 0.0, 0.0);
        let end_coords = (10.0, 10.0, 10.0);

        let path = pathfinder.find_path(start_coords, end_coords);

        println!("{:?}", path);

        assert!(path.is_some());
    }
}
