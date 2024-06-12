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

    pub fn a_star(&self, start: usize, goal: usize, cache: &Mutex<LruCache<(usize, usize), Option<Vec<(Node, u64)>>>>) -> Option<Vec<(Node, u64)>> {
        let cache_key = (start, goal);
        {
            // Check if the path is already in cache
            let mut cache = cache.lock().unwrap();
            if let Some(result) = cache.get(&cache_key) {
                return result.clone();
            }
        }
    
        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<usize, usize> = HashMap::new();
        let mut g_score: HashMap<usize, f64> = HashMap::new();
        let mut f_score: HashMap<usize, f64> = HashMap::new();
        let mut closed_set = HashSet::new();
    
        for &node_id in self.nodes.keys() {
            g_score.insert(node_id, f64::INFINITY);
            f_score.insert(node_id, f64::INFINITY);
        }
    
        g_score.insert(start, 0.0);
        f_score.insert(start, self.heuristic(start, goal));
    
        open_set.push(State {
            cost: 0.0,
            position: start,
        });
    
        while let Some(State { cost: _cost, position: current }) = open_set.pop() {
            if current == goal {
                // Path found
                let mut total_path = vec![];
                let mut total_times = vec![0];
                let mut accumulated_time = 0.0;
                let mut current = current;
    
                while let Some(&next) = came_from.get(&current) {
                    let travel_cost = self.edges.get(&next).unwrap()
                        .iter()
                        .find(|edge| edge.to == current)
                        .unwrap().cost;
                    accumulated_time += travel_cost * 1000.0;
                    total_times.push(accumulated_time as u64);
                    total_path.push((self.nodes[&current], accumulated_time as u64));
                    current = next;
                }
    
                total_path.push((self.nodes[&start], 0));
                total_path.reverse();
    
                let result = Some(total_path.clone());
    
                // Cache the result
                let mut cache = cache.lock().unwrap();
                cache.put(cache_key, result.clone());
    
                return result;
            }
    
            closed_set.insert(current);
    
            if let Some(neighbors) = self.edges.get(&current) {
                for edge in neighbors {
                    let tentative_g_score = g_score[&current] + edge.cost;
    
                    if tentative_g_score < *g_score.get(&edge.to).unwrap_or(&f64::INFINITY) {
                        came_from.insert(edge.to, current);
                        g_score.insert(edge.to, tentative_g_score);
                        f_score.insert(edge.to, tentative_g_score + self.heuristic(edge.to, goal));
                        open_set.push(State {
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
