use crate::graph::Graph;
use crate::node::Node;
use crate::settings::RePathSettings;
use crate::utils::{nodes_within_radius, parse_obj};
use lru::LruCache;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use rand::prelude::*;

/// The RePathfinder struct holds the graph and cache used for pathfinding.
pub struct RePathfinder {
    graph: Graph,
    cache: Arc<Mutex<LruCache<(usize, usize), Option<Vec<(Node, u64)>>>>>,
}

impl RePathfinder {
    /// Creates a new RePathfinder instance with the given settings.
    /// This includes loading the graph from the provided navmesh file and precomputing paths.
    pub fn new(settings: RePathSettings) -> Self {
        let graph = parse_obj(&settings.navmesh_filename);
        let cache_capacity = NonZeroUsize::new(settings.cache_capacity).expect("Capacity must be non-zero");
        let cache = Arc::new(Mutex::new(LruCache::new(cache_capacity)));

        let precompute_start = std::time::Instant::now();
        let node_ids: Vec<_> = graph.nodes.keys().cloned().collect();
        let processed_paths = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        // Precompute paths between random pairs of nodes within a specified radius
        (0..settings.total_precompute_pairs).into_par_iter().for_each(|_| {
            let mut rng = rand::thread_rng();
            let start_node_id = *node_ids.choose(&mut rng).unwrap();
            let start_node = graph.nodes.get(&start_node_id).unwrap();
            let mut nearby_nodes = nodes_within_radius(&graph, start_node, settings.precompute_radius);

            // Remove the start node from the list of nearby nodes if present
            nearby_nodes.retain(|&id| id != start_node_id);

            if let Some(&goal_node_id) = nearby_nodes.choose(&mut rand::thread_rng()) {
                if start_node_id != goal_node_id {
                    graph.a_star(start_node_id, goal_node_id, &cache);
                }
            }

            let count = processed_paths.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
            if count % 100 == 0 {
                println!("Precomputation progress: {:.2}%", (count as f64 / settings.total_precompute_pairs as f64) * 100.0);
            }
        });

        let precompute_duration = precompute_start.elapsed();
        println!("Precomputation time: {:?}", precompute_duration);

        RePathfinder { graph, cache }
    }

    /// Finds a path from start_coords to end_coords using a single thread.
    /// This function uses the A* algorithm and the precomputed cache for efficient pathfinding.
    pub fn find_path(&self, start_coords: (f64, f64, f64), end_coords: (f64, f64, f64)) -> Option<Vec<(Node, u64)>> {
        let start_node_id = self.graph.nearest_node(start_coords.0, start_coords.1, start_coords.2)?;
        let end_node_id = self.graph.nearest_node(end_coords.0, end_coords.1, end_coords.2)?;

        self.graph.a_star(start_node_id, end_node_id, &self.cache)
    }

    /// Finds a path from start_coords to end_coords using multiple threads.
    /// This function splits the pathfinding task into segments, which are processed concurrently.
    ///
    /// Note: Use this function only for long paths. For shorter paths, the overhead of multithreading may result in slower performance compared to the single-threaded version.
    /// Additionally, the resulting path may be slightly different due to the segmentation and concurrent processing.
    pub fn find_path_multithreaded(&self, start_coords: (f64, f64, f64), end_coords: (f64, f64, f64), segment_count: u8) -> Option<Vec<(Node, u64)>> {
        if segment_count <= 1 {
            return self.find_path(start_coords, end_coords);
        }
    
        // Calculate intermediate points
        let mut points = vec![start_coords];
        for i in 1..segment_count {
            let t = i as f64 / segment_count as f64;
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
        let paths: Vec<_> = segments.into_par_iter()
            .map(|segment| {
                let start_node_id = self.graph.nearest_node(segment[0].0, segment[0].1, segment[0].2)?;
                let end_node_id = self.graph.nearest_node(segment[1].0, segment[1].1, segment[1].2)?;
                let path = self.graph.a_star(start_node_id, end_node_id, &self.cache);
                path
            })
            .collect::<Vec<_>>();
    
        // Combine paths
        let mut full_path = Vec::new();
        for path_option in paths {
            if let Some(mut path) = path_option {
                if !full_path.is_empty() {
                    path.remove(0); // Remove duplicate node
                }
                full_path.append(&mut path);
            } else {
                return None; // If any segment fails, the whole path fails
            }
        }
    
        Some(full_path)
    }
}
