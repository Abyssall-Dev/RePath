use crate::graph::Graph;
use crate::node::Node;
use crate::settings::RePathSettings;
use crate::utils::{nodes_within_radius, parse_obj};
use lru::LruCache;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use rand::prelude::*;

pub struct RePathfinder {
    graph: Graph,
    cache: Arc<Mutex<LruCache<(usize, usize), Option<Vec<(Node, u64)>>>>>,
}

impl RePathfinder {
    pub fn new(settings: RePathSettings) -> Self {
        let graph = parse_obj(&settings.navmesh_filename);
        let cache_capacity = NonZeroUsize::new(settings.cache_capacity).expect("Capacity must be non-zero");
        let cache = Arc::new(Mutex::new(LruCache::new(cache_capacity)));

        let precompute_start = std::time::Instant::now();
        let node_ids: Vec<_> = graph.nodes.keys().cloned().collect();
        let processed_paths = Arc::new(std::sync::atomic::AtomicUsize::new(0));

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

    pub fn find_path(&self, start_coords: (f64, f64, f64), end_coords: (f64, f64, f64)) -> Option<Vec<(Node, u64)>> {
        let start_node_id = self.graph.nearest_node(start_coords.0, start_coords.1, start_coords.2)?;
        let end_node_id = self.graph.nearest_node(end_coords.0, end_coords.1, end_coords.2)?;

        self.graph.a_star(start_node_id, end_node_id, &self.cache)
    }
}
