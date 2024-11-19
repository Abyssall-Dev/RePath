use serde::{Serialize, Deserialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Node {
    pub fn new(id: usize, x: f32, y: f32, z: f32) -> Self {
        Node { id, x, y, z }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
