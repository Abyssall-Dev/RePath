use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub to: usize,
    pub cost: f32,
}
