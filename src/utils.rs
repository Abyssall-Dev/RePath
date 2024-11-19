use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};

use crate::graph::Graph;
use crate::metrics::Metrics;
use crate::node::Node;

pub fn parse_obj(filename: &str) -> Graph {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    let mut vertices: Vec<(f32, f32, f32)> = Vec::new();
    let mut vertex_id = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                let x: f32 = parts[1].parse().unwrap();
                let y: f32 = parts[2].parse().unwrap();
                let z: f32 = parts[3].parse().unwrap();
                vertices.push((x, y, z));
                graph.add_node(Node::new(vertex_id, x, y, z));
                vertex_id += 1;
            }
            "f" => {
                let v1 = parts[1].parse::<usize>().unwrap() - 1;
                let v2 = parts[2].parse::<usize>().unwrap() - 1;
                let v3 = parts[3].parse::<usize>().unwrap() - 1;
                graph.add_edge(v1, v2, distance(&vertices[v1], &vertices[v2]));
                graph.add_edge(v2, v3, distance(&vertices[v2], &vertices[v3]));
                graph.add_edge(v3, v1, distance(&vertices[v3], &vertices[v1]));
            }
            _ => {}
        }
    }

    graph
}

pub fn distance(p1: &(f32, f32, f32), p2: &(f32, f32, f32)) -> f32 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let dz = p1.2 - p2.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn nodes_within_radius(graph: &Graph, node: &Node, radius: f32) -> Vec<usize> {
    graph
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(id, n)| {
            let dist = distance(&(node.x, node.y, node.z), &(n.x, n.y, n.z));
            if dist <= radius {
                Some(id)
            } else {
                None
            }
        })
        .collect()
}

pub fn save_metrics_to_csv(
    filename: &str,
    metrics: &Metrics,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_exists = std::path::Path::new(filename).exists();
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(OpenOptions::new().create(true).append(true).open(filename)?);

    if !file_exists {
        wtr.write_record(&[
            "navmesh_filename",
            "use_precomputed_cache",
            "precompute_radius",
            "total_paths_precomputed",
            "total_precompute_pairs",
            "precomputation_time",
            "pathfinding_time",
        ])?;
    }

    wtr.write_record(&[
        &metrics.settings.navmesh_filename,
        &metrics.settings.use_precomputed_cache.to_string(),
        &metrics.settings.precompute_radius.to_string(),
        &metrics.total_paths_precomputed.to_string(),
        &metrics.settings.total_precompute_pairs.to_string(),
        &metrics.precomputation_time.to_string(),
        &metrics.pathfinding_time.to_string(),
    ])?;
    wtr.flush()?;
    Ok(())
}
