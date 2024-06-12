use repath::utils::{nodes_within_radius, parse_obj};
use repath::settings::RePathSettings;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;
use rand::prelude::SliceRandom;

#[test]
fn test_pathfinding() {
    let settings = RePathSettings {
        navmesh_filename: "navmesh_varied.obj".to_string(),
        precompute_radius: 50.0,
        total_precompute_pairs: 100,
        cache_capacity: 100,
        use_precomputed_cache: true,
    };

    let graph = parse_obj(&settings.navmesh_filename);
    let cache_capacity = NonZeroUsize::new(settings.cache_capacity).expect("Capacity must be non-zero");
    let cache = Mutex::new(LruCache::new(cache_capacity));

    let start_node_id = graph.random_node().expect("No nodes found in graph");
    let start_node = graph.nodes.get(&start_node_id).unwrap();
    let mut nearby_nodes = nodes_within_radius(&graph, start_node, 50.0);
    nearby_nodes.retain(|&id| id != start_node_id);
    let goal_node_id = *nearby_nodes.choose(&mut rand::thread_rng()).expect("No nearby nodes found");

    let path = graph.a_star(start_node_id, goal_node_id, &cache);
    assert!(path.is_some());
}
