use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::Arc;
use dashmap::DashMap;
use rand::prelude::*;
use crate::edge::Edge;
use crate::node::Node;
use crate::path::Path;
use crate::utils::distance;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        self.edges.push(Vec::new());
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: f32) {
        self.edges[from].push(Edge { to, cost });
    }

    pub fn heuristic(&self, start: usize, goal: usize) -> f32 {
        let start_node = &self.nodes[start];
        let goal_node = &self.nodes[goal];
        let dx = start_node.x - goal_node.x;
        let dy = start_node.y - goal_node.y;
        let dz = start_node.z - goal_node.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }


    pub fn a_star(
        &self,
        start: usize,
        goal: usize,
        cache: &DashMap<(usize, usize), Option<Path>>,
    ) -> Option<Path> {
        let cache_key = (start, goal);

        // Check if the path is already in cache
        if let Some(result) = cache.get(&cache_key) {
            return result.clone();
        }

        let num_nodes = self.nodes.len();
        let mut open_set = BinaryHeap::with_capacity(num_nodes);
        let mut came_from = vec![None; num_nodes];
        let mut g_score = vec![f32::INFINITY; num_nodes];
        let mut f_score = vec![f32::INFINITY; num_nodes];
        let mut closed_set = vec![false; num_nodes];

        g_score[start] = 0.0;
        f_score[start] = self.heuristic(start, goal);

        open_set.push(State {
            cost: f_score[start],
            position: start,
        });

        while let Some(State { cost: _, position: current }) = open_set.pop() {
            if current == goal {
                // Path found
                let mut total_path = Vec::new();
                let mut current = current;

                total_path.push(self.nodes[current]);

                while let Some(next) = came_from[current] {
                    total_path.push(self.nodes[next]);
                    current = next;
                }

                total_path.reverse();

                let result = Some(Arc::new(total_path));

                // Cache the result
                cache.insert(cache_key, result.clone());

                return result;
            }

            if closed_set[current] {
                continue;
            }
            closed_set[current] = true;

            for edge in &self.edges[current] {
                let neighbor = edge.to;

                if closed_set[neighbor] {
                    continue;
                }

                let tentative_g_score = g_score[current] + edge.cost;

                if tentative_g_score < g_score[neighbor] {
                    came_from[neighbor] = Some(current);
                    g_score[neighbor] = tentative_g_score;
                    f_score[neighbor] = tentative_g_score + self.heuristic(neighbor, goal);
                    open_set.push(State {
                        cost: f_score[neighbor],
                        position: neighbor,
                    });
                }
            }
        }

        // Cache the non-result
        cache.insert(cache_key, None);

        None
    }

    pub fn nearest_node(&self, x: f32, y: f32, z: f32) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(id, node)| {
                let d = distance(&(node.x, node.y, node.z), &(x, y, z));
                (d, id)
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|(_, id)| id)
    }

    pub fn random_node(&self) -> Option<usize> {
        let node_ids: Vec<_> = (0..self.nodes.len()).collect();
        if node_ids.is_empty() {
            None
        } else {
            let mut rng = thread_rng();
            Some(*node_ids.choose(&mut rng).unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub cost: f32,
    pub position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}
