pub mod node;
pub mod edge;
pub mod graph;
pub mod metrics;
mod path;
pub mod pathfinder;
pub mod settings;
pub mod utils;

pub use pathfinder::RePathfinder;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::settings::RePathSettings;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn test_pathfinding() {
        // Create a new RePathSettings instance with custom settings
        let settings = RePathSettings {
            navmesh_filename: "NavMesh.obj".to_string(),
            precompute_radius: 10000.0,
            total_precompute_pairs: 5000,
            use_precomputed_cache: true,
        };

        // Create a new RePathfinder instance
        let pathfinder = RePathfinder::new(settings);

        // Optionally, print the graph bounds
        fn print_graph_bounds(graph: &Graph) {
            let mut min_x = f32::MAX;
            let mut min_y = f32::MAX;
            let mut min_z = f32::MAX;
            let mut max_x = f32::MIN;
            let mut max_y = f32::MIN;
            let mut max_z = f32::MIN;

            for node in &graph.nodes {
                if node.x < min_x { min_x = node.x; }
                if node.y < min_y { min_y = node.y; }
                if node.z < min_z { min_z = node.z; }
                if node.x > max_x { max_x = node.x; }
                if node.y > max_y { max_y = node.y; }
                if node.z > max_z { max_z = node.z; }
            }

            println!("Graph bounds:");
            println!("X: {} to {}", min_x, max_x);
            println!("Y: {} to {}", min_y, max_y);
            println!("Z: {} to {}", min_z, max_z);
        }

        // Print the graph bounds
        print_graph_bounds(&pathfinder.graph);

        // Find a non-isolated start node
        let start_node_id = find_non_isolated_start_node(&pathfinder.graph)
            .expect("Could not find a non-isolated start node");
        let start_node = &pathfinder.graph.nodes[start_node_id];
        let start_coords = (start_node.x, start_node.y, start_node.z);

        println!(
            "Selected start node ID: {}, Position: {:?}",
            start_node_id, start_node
        );

        // Print edges of the start node
        println!(
            "Edges from start node (ID: {}): {:?}",
            start_node_id,
            pathfinder.graph.edges[start_node_id]
        );
        println!(
            "Number of edges from start node: {}",
            pathfinder.graph.edges[start_node_id].len()
        );

        // Find a node connected to the start node
        let end_node_id = find_connected_node(&pathfinder.graph, start_node_id)
            .expect("Could not find a node connected to the start node");
        let end_node = &pathfinder.graph.nodes[end_node_id];

        println!(
            "Selected end node ID: {}, Position: {:?}",
            end_node_id, end_node
        );

        // Update end_coords to match the selected end node
        let end_coords = (end_node.x, end_node.y, end_node.z);

        // Confirm connectivity
        let connected = are_nodes_connected(&pathfinder.graph, start_node_id, end_node_id);
        assert!(connected, "Start and end nodes are not connected");

        // Find path using a single thread
        let start_time = std::time::Instant::now();
        let path1 = pathfinder.find_path(start_coords, end_coords);
        println!("Time to find path single-threaded: {:?}", start_time.elapsed());
        if let Some(path) = &path1 {
            println!("Path found with {} nodes.", path.len());
        } else {
            println!("No path found between start_coords and end_coords");
        }

        assert!(path1.is_some(), "No path found between start_coords and end_coords");

        // Find path using multiple threads
        let start_time = std::time::Instant::now();
        let path2 = pathfinder.find_path_multithreaded(start_coords, end_coords, 4);
        println!("Time to find path multi-threaded: {:?}", start_time.elapsed());

        if let Some(path) = &path2 {
            println!("Multithreaded path found with {} nodes.", path.len());
        } else {
            println!("No path found between start_coords and end_coords using multithreaded pathfinding");
        }

        assert!(
            path2.is_some(),
            "No path found between start_coords and end_coords with multithreaded pathfinding"
        );
    }

    fn find_non_isolated_start_node(graph: &Graph) -> Option<usize> {
        for (node_id, edges) in graph.edges.iter().enumerate() {
            if !edges.is_empty() {
                return Some(node_id);
            }
        }
        None
    }

    fn find_connected_node(graph: &Graph, start_node_id: usize) -> Option<usize> {
        let connected_nodes: Vec<usize> = graph.edges[start_node_id].iter().map(|edge| edge.to).collect();
        if connected_nodes.is_empty() {
            // Start node is isolated
            return None;
        }
        let mut rng = thread_rng();
        let &potential_goal = connected_nodes.choose(&mut rng)?;
        Some(potential_goal)
    }

    use std::collections::VecDeque;

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
}
