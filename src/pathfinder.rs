use crate::graph::Graph;
use crate::settings::RePathSettings;
use crate::utils::{nodes_within_radius, parse_obj};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;
use dashmap::DashMap;
use rand::prelude::*;
use crate::path::Path;

/// The RePathfinder struct holds the graph and cache used for pathfinding.
pub struct RePathfinder {
    pub(crate) graph: Graph,
    cache: Arc<DashMap<(usize, usize), Option<Path>>>,
}

impl RePathfinder {
    /// Creates a new RePathfinder instance with the given settings.
    /// This includes loading the graph from the provided navmesh file and precomputing paths.
    pub fn new(settings: RePathSettings) -> Self {
        let graph = parse_obj(&settings.navmesh_filename);
        let cache = Arc::new(DashMap::new());

        let precompute_start = std::time::Instant::now();
        let node_ids: Vec<_> = (0..graph.nodes.len()).collect();

        // Precompute paths between random pairs of nodes within a specified radius
        (0..settings.total_precompute_pairs)
            .into_par_iter()
            .for_each(|_| {
                let mut rng = rand::thread_rng();
                let start_node_id = *node_ids.choose(&mut rng).unwrap();
                let start_node = &graph.nodes[start_node_id];
                let mut nearby_nodes =
                    nodes_within_radius(&graph, start_node, settings.precompute_radius);

                // Remove the start node from the list of nearby nodes if present
                nearby_nodes.retain(|&id| id != start_node_id);

                if let Some(&goal_node_id) = nearby_nodes.choose(&mut rng) {
                    if start_node_id != goal_node_id {
                        graph.a_star(start_node_id, goal_node_id, &cache);
                    }
                }
            });

        let precompute_duration = precompute_start.elapsed();
        println!("Precomputation time: {:?}", precompute_duration);

        RePathfinder { graph, cache }
    }

    /// Finds a path from start_coords to end_coords.
    pub fn find_path(&self, start_coords: (f32, f32, f32), end_coords: (f32, f32, f32)) -> Option<Path> {
        let start_node_id = self.graph.nearest_node(start_coords.0, start_coords.1, start_coords.2)?;
        let end_node_id = self.graph.nearest_node(end_coords.0, end_coords.1, end_coords.2)?;

        self.graph.a_star(start_node_id, end_node_id, &self.cache)
    }

    /// Finds a path from start_coords to end_coords using multiple threads.
    /// This function splits the pathfinding task into segments, which are processed concurrently.
    pub fn find_path_multithreaded(
        &self,
        start_coords: (f32, f32, f32),
        end_coords: (f32, f32, f32),
        segment_count: u8,
    ) -> Option<Path> {
        if segment_count <= 1 {
            return self.find_path(start_coords, end_coords);
        }

        // Calculate intermediate points
        let mut points = vec![start_coords];
        for i in 1..segment_count {
            let t = i as f32 / segment_count as f32;
            let intermediate_point = (
                start_coords.0 + t * (end_coords.0 - start_coords.0),
                start_coords.1 + t * (end_coords.1 - start_coords.1),
                start_coords.2 + t * (end_coords.2 - start_coords.2),
            );
            points.push(intermediate_point);
        }
        points.push(end_coords);

        // Create tasks for each segment
        let segments: Vec<_> = points.windows(2).collect();
        let paths: Vec<_> = segments
            .into_par_iter()
            .map(|segment| {
                let start_node_id = self
                    .graph
                    .nearest_node(segment[0].0, segment[0].1, segment[0].2)?;
                let end_node_id = self
                    .graph
                    .nearest_node(segment[1].0, segment[1].1, segment[1].2)?;
                self.graph.a_star(start_node_id, end_node_id, &self.cache)
            })
            .collect();

        // Combine paths
        let mut full_path = Vec::new();
        for path_option in paths {
            if let Some(path) = path_option {
                if !full_path.is_empty() {
                    full_path.pop(); // Remove duplicate node
                }
                full_path.extend(path.iter());
            } else {
                return None; // If any segment fails, the whole path fails
            }
        }

        Some(Arc::new(full_path))
    }
}
