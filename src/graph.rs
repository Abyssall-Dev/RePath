use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::Mutex;
use lru::LruCache;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use crate::node::Node;
use crate::edge::Edge;
use crate::utils::distance;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<usize, Node>,
    pub edges: HashMap<usize, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: usize, x: f64, y: f64, z: f64) {
        let node = Node::new(id, x, y, z);
        self.nodes.insert(id, node);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cost: f64) {
        let edge = Edge { to, cost };
        self.edges.entry(from).or_insert(Vec::new()).push(edge);
    }

    pub fn heuristic(&self, start: usize, goal: usize) -> f64 {
        let start_node = self.nodes.get(&start).unwrap();
        let goal_node = self.nodes.get(&goal).unwrap();
        let dx = start_node.x - goal_node.x;
        let dy = start_node.y - goal_node.y;
        let dz = start_node.z - goal_node.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn bi_directional_a_star(&self, start: usize, goal: usize, cache: &Mutex<LruCache<(usize, usize), Option<Vec<(Node, u64)>>>>) -> Option<Vec<(Node, u64)>> {
        let cache_key = (start, goal);
        {
            // Check if the path is already in cache
            let mut cache = cache.lock().unwrap();
            if let Some(result) = cache.get(&cache_key) {
                return result.clone();
            }
        }

        let mut open_set_fwd = BinaryHeap::new();
        let mut open_set_bwd = BinaryHeap::new();
        let mut came_from_fwd: HashMap<usize, usize> = HashMap::new();
        let mut came_from_bwd: HashMap<usize, usize> = HashMap::new();
        let mut g_score_fwd: HashMap<usize, f64> = HashMap::new();
        let mut g_score_bwd: HashMap<usize, f64> = HashMap::new();
        let mut f_score_fwd: HashMap<usize, f64> = HashMap::new();
        let mut f_score_bwd: HashMap<usize, f64> = HashMap::new();
        let mut closed_set_fwd = HashSet::new();
        let mut closed_set_bwd = HashSet::new();

        for &node_id in self.nodes.keys() {
            g_score_fwd.insert(node_id, f64::INFINITY);
            g_score_bwd.insert(node_id, f64::INFINITY);
            f_score_fwd.insert(node_id, f64::INFINITY);
            f_score_bwd.insert(node_id, f64::INFINITY);
        }

        g_score_fwd.insert(start, 0.0);
        g_score_bwd.insert(goal, 0.0);
        f_score_fwd.insert(start, self.heuristic(start, goal));
        f_score_bwd.insert(goal, self.heuristic(goal, start));

        open_set_fwd.push(State {
            cost: 0.0,
            position: start,
        });

        open_set_bwd.push(State {
            cost: 0.0,
            position: goal,
        });

        while let (Some(State { cost: _cost_fwd, position: pos_fwd }), Some(State { cost: _cost_bwd, position: pos_bwd })) = (open_set_fwd.pop(), open_set_bwd.pop()) {
            if closed_set_fwd.contains(&pos_bwd) || closed_set_bwd.contains(&pos_fwd) {
                // Path found
                let mut total_path_fwd = vec![];
                let mut total_path_bwd = vec![];
                let mut total_times_fwd = vec![0];
                let mut total_times_bwd = vec![0];
                let mut current = pos_fwd;
                let mut accumulated_time = 0.0;

                while let Some(&next) = came_from_fwd.get(&current) {
                    let travel_cost = self.edges.get(&next).unwrap()
                        .iter()
                        .find(|edge| edge.to == current)
                        .unwrap().cost;
                    accumulated_time += travel_cost * 1000.0;
                    total_times_fwd.push(accumulated_time as u64);
                    total_path_fwd.push((self.nodes[&current], accumulated_time as u64));
                    current = next;
                }

                total_path_fwd.push((self.nodes[&start], 0));
                total_path_fwd.reverse();

                current = pos_bwd;
                accumulated_time = 0.0;

                while let Some(&next) = came_from_bwd.get(&current) {
                    let travel_cost = self.edges.get(&next).unwrap()
                        .iter()
                        .find(|edge| edge.to == current)
                        .unwrap().cost;
                    accumulated_time += travel_cost * 1000.0;
                    total_times_bwd.push(accumulated_time as u64);
                    total_path_bwd.push((self.nodes[&current], accumulated_time as u64));
                    current = next;
                }

                total_path_bwd.push((self.nodes[&goal], 0));

                total_path_fwd.extend(total_path_bwd.iter().rev().cloned());
                let result = Some(total_path_fwd.clone());

                // Cache the result
                let mut cache = cache.lock().unwrap();
                cache.put(cache_key, result.clone());

                // Also cache the inverted path
                let inverted_path: Vec<(Node, u64)> = total_path_fwd.iter().rev().cloned().collect();
                let inverted_cache_key = (goal, start);
                cache.put(inverted_cache_key, Some(inverted_path));

                return result;
            }

            closed_set_fwd.insert(pos_fwd);
            closed_set_bwd.insert(pos_bwd);

            if let Some(neighbors) = self.edges.get(&pos_fwd) {
                for edge in neighbors {
                    let tentative_g_score = g_score_fwd[&pos_fwd] + edge.cost;

                    if tentative_g_score < *g_score_fwd.get(&edge.to).unwrap_or(&f64::INFINITY) {
                        came_from_fwd.insert(edge.to, pos_fwd);
                        g_score_fwd.insert(edge.to, tentative_g_score);
                        f_score_fwd.insert(edge.to, tentative_g_score + self.heuristic(edge.to, goal));
                        open_set_fwd.push(State {
                            cost: tentative_g_score,
                            position: edge.to,
                        });
                    }
                }
            }

            if let Some(neighbors) = self.edges.get(&pos_bwd) {
                for edge in neighbors {
                    let tentative_g_score = g_score_bwd[&pos_bwd] + edge.cost;

                    if tentative_g_score < *g_score_bwd.get(&edge.to).unwrap_or(&f64::INFINITY) {
                        came_from_bwd.insert(edge.to, pos_bwd);
                        g_score_bwd.insert(edge.to, tentative_g_score);
                        f_score_bwd.insert(edge.to, tentative_g_score + self.heuristic(edge.to, start));
                        open_set_bwd.push(State {
                            cost: tentative_g_score,
                            position: edge.to,
                        });
                    }
                }
            }
        }

        let result = None;

        // Cache the result
        let mut cache = cache.lock().unwrap();
        cache.put(cache_key, result.clone());

        result
    }

    pub fn nearest_node(&self, x: f64, y: f64, z: f64) -> Option<usize> {
        self.nodes.iter()
            .min_by(|(_, a), (_, b)| {
                let da = distance(&(a.x, a.y, a.z), &(x, y, z));
                let db = distance(&(b.x, b.y, b.z), &(x, y, z));
                da.partial_cmp(&db).unwrap()
            })
            .map(|(&id, _)| id)
    }

    pub fn random_node(&self) -> Option<usize> {
        let node_ids: Vec<_> = self.nodes.keys().cloned().collect();
        if node_ids.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            Some(*node_ids.choose(&mut rng).unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct State {
    pub cost: f64,
    pub position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
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
