use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Vec<String>>, // Maps a node to its neighbors
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        self.nodes.entry(node1.to_string())
            .or_insert_with(Vec::new)
            .push(node2.to_string());
    }

    pub fn get_neighbors(&self, node: &str) -> Option<&Vec<String>> {
        self.nodes.get(node)
    }
}