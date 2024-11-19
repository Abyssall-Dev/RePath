use std::collections::VecDeque;
use repath::graph::Graph;
use repath::settings::RePathSettings;
use repath::utils::parse_obj;
use dashmap::DashMap;

#[test]
fn test_pathfinding_connected_nodes() {
    let settings = RePathSettings {
        navmesh_filename: "NavMesh.obj".to_string(),
        precompute_radius: 5000.0,
        total_precompute_pairs: 100,
        use_precomputed_cache: true,
    };

    // Parse the navmesh file into a graph
    let graph = parse_obj(&settings.navmesh_filename);

    // Initialize the cache using DashMap for concurrent access
    let cache = DashMap::new();

    // Find connected nodes
    let (start_node_id, goal_node_id) = find_connected_nodes(&graph).expect("No connected nodes found");

    // Print node information for debugging
    println!(
        "Start node ID: {}, Position: {:?}",
        start_node_id, graph.nodes[start_node_id]
    );
    println!(
        "Goal node ID: {}, Position: {:?}",
        goal_node_id, graph.nodes[goal_node_id]
    );

    // Run the A* algorithm to find a path between the start and goal nodes
    let path = graph.a_star(start_node_id, goal_node_id, &cache);

    // Assert that a path was found
    assert!(path.is_some(), "No path found between start and goal nodes");
}

fn find_connected_nodes(graph: &Graph) -> Option<(usize, usize)> {
    for start_node_id in 0..graph.nodes.len() {
        for goal_node_id in (start_node_id + 1)..graph.nodes.len() {
            if are_nodes_connected(graph, start_node_id, goal_node_id) {
                return Some((start_node_id, goal_node_id));
            }
        }
    }
    None
}

fn are_nodes_connected(graph: &Graph, start: usize, goal: usize) -> bool {
    let mut visited = vec![false; graph.nodes.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            return true;
        }
        if visited[current] {
            continue;
        }
        visited[current] = true;
        for edge in &graph.edges[current] {
            if !visited[edge.to] {
                queue.push_back(edge.to);
            }
        }
    }
    false
}
