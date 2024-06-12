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
    use super::*;

    #[test]
    fn test_pathfinding() {
        let settings = settings::RePathSettings {
            navmesh_filename: "navmesh_varied.obj".to_string(),
            precompute_radius: 50.0,
            total_precompute_pairs: 100,
            cache_capacity: 100,
            use_precomputed_cache: true,
        };

        let pathfinder = RePathfinder::new("navmesh_varied.obj", settings);

        let start_coords = (0.0, 0.0, 0.0);
        let end_coords = (10.0, 10.0, 10.0);

        let path = pathfinder.find_path(start_coords, end_coords);

        println!("{:?}", path);

        assert!(path.is_some());
    }
}
